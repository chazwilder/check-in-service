use serde::{Serialize, Deserialize};
use anyhow;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, FromStr)]
pub struct TPTString(Option<String>);

impl TPTString {
    pub fn new<T: Into<Option<String>>>(s: T) -> Self {
        let content = s.into();
        match content {
            Some(content) => {
                if !content.trim().is_empty() {
                    Ok(Self(Some(content)))
                } else {
                    Ok(Self(None))
                }
            }
        None => Ok(Self(None))
        }
    }
}