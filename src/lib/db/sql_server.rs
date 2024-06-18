use futures::TryStreamExt;
use tiberius::{AuthMethod, Client, Config, QueryItem, Row};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use dotenvy::dotenv;
use std::env;
use crate::interfaces::NewOrder;


pub async fn get_new_orders() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let mut config = Config::new();
    config.host(env::var("SERVER_NAME").unwrap_or_default());
    config.database(env::var("DATABASE_NAME").unwrap_or_default());
    config.port(1433);
    config.authentication(AuthMethod::Integrated);
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let sql = env::var("CHECKIN_QUERY").unwrap_or_default();

    let mut stream = client
        .query(&sql,&[]).await?;
    while let Some(item) = stream.try_next().await? {
    match item {
        QueryItem::Row(row) => {
            let new_order= NewOrder::from_row(&row).unwrap();
            //println!("{:?}", row);
            println!("{:?}", new_order);
        }
        _ => {}
    }
    }
    Ok(())
}