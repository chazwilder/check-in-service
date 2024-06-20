use sqlx_oldapi::mssql::MssqlPoolOptions;
use sqlx_oldapi::Row;
use sqlx_oldapi::types::chrono::NaiveDateTime;
use futures::TryStreamExt;
use sqlx_oldapi::Connection;
use dotenvy::dotenv;
use std::env;
use crate::interfaces::NewOrder;


pub async fn get_new_orders() -> Result<Vec<NewOrder>, anyhow::Error> {
    dotenv().ok();
    let pool = MssqlPoolOptions::new()
        .max_connections(5)
        .connect(env::var("MSSQL_URL").unwrap_or_default().as_str()).await?;
    let sql = env::var("CHECKIN_QUERY").unwrap_or_default();

    let mut rows = sqlx_oldapi::query_as::<_,NewOrder>(&sql).fetch_all(&pool).await?;
    for row in &rows {
        println!("{:#?}", row);
    }
    Ok(rows)
}