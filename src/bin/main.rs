use check_in::db::sql_server;
use tokio;
#[tokio::main]
async fn main() {
    sql_server::get_new_orders().await.unwrap();
}
