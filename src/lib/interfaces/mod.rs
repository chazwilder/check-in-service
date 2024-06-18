pub mod types;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct NewOrder {
    pub TC_SHIPMENT_ID: types::TripNumber,
    pub D_ADDRESS: types::TPTString,
    pub D_CITY: types::TPTString,
    pub D_STATE_PROV: types::TPTString,
    pub D_POSTAL_CODE: types::TPTInt,
    pub NUM_STOPS: types:: TPTInt,
    pub PICKUP_START_DTTM: types::TPTDateTime,
    pub PRELOAD: types::TPTString,
    pub CUSTOMER_NAME: types::TPTString,
    pub TRAILER_NUMBER: types::TPTString,
    pub CREATED_SOURCE_DTTM: types::TPTDateTime,
    pub ACTUAL_CHECKIN_DTTM: types::TPTDateTime,
    pub YARD_ACTIVITY_DTTM: types::TPTDateTime,
    pub APPOINTMENT_ID: types::TPTInt,
    pub TRAILER_REF_ID: types::TPTInt,
    pub ACTIVITY_TYPE: types::TPTInt,
    pub ACTIVITY_USER: types::TPTString,
}