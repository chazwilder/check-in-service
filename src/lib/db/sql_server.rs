use sqlx_oldapi::mssql::MssqlPoolOptions;
use dotenvy::dotenv;
use std::env;
use crate::interfaces::NewOrder;
use log::{info, error};


pub async fn get_new_orders() -> Result<Vec<NewOrder>, anyhow::Error> {
    dotenv().ok();
    let pool = MssqlPoolOptions::new()
        .max_connections(5)
        .connect(env::var("MSSQL_URL").unwrap_or_default().as_str()).await?;
    let sql = env::var("CHECKIN_QUERY").unwrap_or_default();

    match sqlx_oldapi::query_as::<_, NewOrder>(&sql).fetch_all(&pool).await {
        Ok(rows) => {
            info!("Retrieved {} new orders from SQL Server", rows.len());
            Ok(rows)
        }
        Err(err) => {
            error!("Error retrieving new orders from SQL Server: {:?}", err);
            Err(err.into())
        }
    }
}