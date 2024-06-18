use tokio;
use dotenvy::dotenv;
use std::env;
use futures::stream::TryStreamExt;
use mongodb::{Client, options::ClientOptions, options::FindOptions};
use mongodb::bson::{doc, Document};





pub async fn get_sites()-> Result<(), anyhow::Error> {
    dotenv().ok();
    let url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL must be set");
    let database = env::var("MONGO_DATABASE").expect("MONGO_DATABASE");
    let collection_name = env::var("MONGO_COLLECTION").expect("MONGO_COLLECTION");
    let client_options = ClientOptions::parse(&url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&database);
    let collection = db.collection::<Document>(&collection_name);
    let mut cursor = collection.find(Some(doc! {}), Some(FindOptions::builder().build())).await?;
    let mut data = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        let site = mongodb::bson::from_bson(mongodb::bson::Bson::Document(result))?;
        data.push(site);
    }
    return Ok(());
}