use dotenvy::dotenv;
use std::env;
use futures::stream::TryStreamExt;
use chrono::{Utc};
use mongodb::{Client, options::ClientOptions, options::FindOptions};
use mongodb::bson::{doc, Document, to_bson};
use crate::interfaces::{NewOrder,MongoShipments};
use log::{info, error};



pub async fn get_orders()-> Result<(), anyhow::Error> {
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

pub async fn add_order(shipment: NewOrder)-> Result<(), anyhow::Error> {
    dotenv().ok();
    let url = env::var("MONGO_DB_URL").expect("MONGO_DB_URL must be set");
    let database = env::var("MONGO_DATABASE").expect("MONGO_DATABASE");
    let collection_name = env::var("MONGO_COLLECTION").expect("MONGO_COLLECTION");
    let client_options = ClientOptions::parse(&url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&database);
    let collection = db.collection::<Document>(&collection_name);
    let new_shipment =  MongoShipments {
        CREATED_DTTM: Some(Utc::now()),
        CHECKIN_DTTM: Some(shipment.ACTUAL_CHECKIN_DTTM.and_utc()),
        TRAILER_INSPECTED_DTTM: None,
        FIRST_DROP_DTTM: None,
        LOADED_DTTM: None,
        CHECKOUT_DTTM: None,
        MA_SHIPMENT_ID: Some(shipment.TC_SHIPMENT_ID),
        SDM_SHIPMENT_ID: None,
        TRIP_NUMBER: shipment.TC_SHIPMENT_ID,
        DESTINATION: Some(format!("{}, {}, {} {}", shipment.D_ADDRESS, shipment.D_CITY, shipment.D_STATE_PROV, shipment.D_POSTAL_CODE)),
        CUSTOMER: shipment.CUSTOMER_NAME,
        CARRIER: None,
        APPOINTMENT_DTTM: Option::from(shipment.PICKUP_START_DTTM.and_utc()),
        APPOINTMENT_ADHERENCE: None,
        GRANT_DETENTION: None,
        SKU: None,
        SKU_LOCATION_COUNT: None,
        PALLET_COUNT: None,
        PLANT_ASSETS: None,
        LOCATIONS: None,
        AGING_LPNS: None,
        MULTISTOP_COUNT: Some(shipment.NUM_STOPS),
        LOAD_TYPE: None,
        LOAD_TIME: None,
        PROCESS_TIME: None,
        GATE_TO_DOCK: None,
        REDOCKED: None,
        DOCK_TO_INSPECTED: None,
        START_TO_FIRST_DROP: None,
        PRODUCTION_BLOCKS: None,
        LOAD_PATTERN: None,
        TIME_MACHINE: None,
        APPLICATION_SETTING: None,
        CROSSDOCKING_ENABLED: None,
        LOADED_LPNS: None,
    };
    let doc = to_bson(&new_shipment)?.as_document().unwrap().clone();
    match collection.insert_one(doc, None).await {
        Ok(_) => {
            info!("Order added successfully: trip_number={}", shipment.TC_SHIPMENT_ID);
            Ok(())
        }
        Err(err) => {
            error!("Error adding order: trip_number={}, error={}", shipment.TC_SHIPMENT_ID, err.to_string());
            Err(err.into())
        }
    }
}