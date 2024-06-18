use serde::{Serialize, Deserialize};
use derive_more::{Constructor, FromStr, Eq, PartialEq};


#[derive(Serialize, Deserialize, Debug, Clone, FromStr, Constructor, Eq, PartialEq)]
pub struct TPTInt(i64);