use check_in::db::{sql_server, mongodb};
use check_in::heartbeat::configure_heartbeat;
use actix_web::{App, HttpServer};
use tokio;
use log4rs;
use log::{info, error};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(AtomicUsize::new(0));
    log4rs::init_file("C:\\Users\\cwilder\\Desktop\\dev\\TPT\\check_in_service\\log4rs.yaml", Default::default()).unwrap();

    tokio::spawn(async {
        HttpServer::new(|| {
            App::new().configure(configure_heartbeat)
        })
        .bind("127.0.0.1:3032")
        .expect("Failed to bind server")
        .run()
        .await
        .expect("Failed to run server");
    });

    let counter_clone = Arc::clone(&counter);
    tokio::spawn(async move {
        loop {
            let current_count = counter_clone.fetch_add(1, Ordering::SeqCst);
            info!("Query IDX: {:?}", current_count + 1);
            match sql_server::get_new_orders().await {
                Ok(new_orders) => {
                    for order in new_orders {
                        if let Err(err) = mongodb::add_order(order).await {
                            error!("Failed to add order: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    error!("Failed to get new orders: {:?}", err);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    });

    tokio::signal::ctrl_c().await?;
    println!("Shutting down gracefully...");
    Ok(())
}
