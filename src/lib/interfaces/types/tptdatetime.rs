use std::str::FromStr;
use serde::{Serialize, Deserialize};
use derive_more::{Constructor, From};
use chrono::{DateTime, NaiveDateTime, Local};


#[derive(Serialize, Deserialize, Debug, Clone, From, Constructor)]
pub struct TPTDateTime(DateTime<Local>);

impl TPTDateTime {
    pub fn into_inner(self) -> DateTime<Local> {
        self.0
    }
    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }
}

impl FromStr for TPTDateTime {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?.into()))
    }
}