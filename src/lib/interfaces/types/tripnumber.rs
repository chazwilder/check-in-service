use serde::{Serialize, Deserialize};
use derive_more::{Constructor, FromStr};


#[derive(Serialize, Deserialize, Debug, Clone, FromStr, Constructor, Eq, PartialEq)]
pub struct TripNumber(i64);