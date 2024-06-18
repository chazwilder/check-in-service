use chrono::{NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct NewOrder {
    pub TC_SHIPMENT_ID: i32,
    pub D_ADDRESS: String,
    pub D_CITY: String,
    pub D_STATE_PROV: String,
    pub D_POSTAL_CODE: i32,
    pub NUM_STOPS: i32,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub PICKUP_START_DTTM: NaiveDateTime,
    pub PRELOAD: String,
    pub CUSTOMER_NAME: String,
    pub TRAILER_NUMBER: String,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub CREATED_SOURCE_DTTM: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub ACTUAL_CHECKIN_DTTM: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_dttm", serialize_with = "serialize_dttm")]
    pub YARD_ACTIVITY_DTTM: NaiveDateTime,
    pub APPOINTMENT_ID: i32,
    pub TRAILER_REF_ID: i32,
    pub ACTIVITY_TYPE: i32,
    pub ACTIVITY_USER: String,
}
impl NewOrder {
    pub fn from_row(row: &tiberius::Row) -> Result<Self, tiberius::error::Error> {
        Ok(Self {
            TC_SHIPMENT_ID: row.try_get::<&str, _>("TC_SHIPMENT_ID")
                .ok()
                .flatten()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            D_ADDRESS: row.try_get::<&str, _>("D_ADDRESS")
                .ok()
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            D_CITY: row.try_get::<&str, _>("D_CITY")
                .ok()
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            D_STATE_PROV: row.try_get::<&str, _>("D_STATE_PROV")
                .ok()
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            D_POSTAL_CODE: row.try_get::<&str, _>("D_POSTAL_CODE")
                .ok()
                .flatten()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            NUM_STOPS: row.try_get::<&str, _>("NUM_STOPS")
                .ok()
                .flatten()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            PICKUP_START_DTTM: row.try_get::<NaiveDateTime, _>("PICKUP_START_DTTM")
                .ok()
                .flatten()
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0)),
            PRELOAD: row.try_get::<&str, _>("PRELOAD")
                .ok()
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            CUSTOMER_NAME: row.try_get::<&str, _>("CUSTOMER_NAME")
                .ok()
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            TRAILER_NUMBER: row.try_get::<&str, _>("TRAILER_NUMBER")
                .ok()
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_default(),
            CREATED_SOURCE_DTTM: row.try_get::<NaiveDateTime, _>("CREATED_SOURCE_DTTM")
                .ok()
                .flatten()
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0)),
            ACTUAL_CHECKIN_DTTM: row.try_get::<NaiveDateTime, _>("ACTUAL_CHECKIN_DTTM")
                .ok()
                .flatten()
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0)),
            YARD_ACTIVITY_DTTM: row.try_get::<NaiveDateTime, _>("YARD_ACTIVITY_DTTM")
                .ok()
                .flatten()
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0)),
            APPOINTMENT_ID: row.try_get::<&str, _>("APPOINTMENT_ID")
                .ok()
                .flatten()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            TRAILER_REF_ID: row.try_get::<&str, _>("TRAILER_REF_ID")
                .ok()
                .flatten()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            ACTIVITY_TYPE: row.try_get::<&str, _>("ACTIVITY_TYPE")
                .ok()
                .flatten()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            ACTIVITY_USER: row.try_get::<&str, _>("ACTIVITY_USER")
                .ok()
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_default(),
        })
    }
}



const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
fn deserialize_dttm<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}

fn serialize_dttm<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}