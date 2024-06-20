use check_in::db::{sql_server, mongodb};
use tokio;

#[tokio::main]
async fn main() {
    //loop {
        let new_orders = sql_server::get_new_orders().await.unwrap();
        for order in new_orders {
            mongodb::add_order(order).await.unwrap();
        }
        //tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    //}
}
