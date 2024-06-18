use serde::{Serialize, Deserialize};
use anyhow;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPTString(Option<String>);

impl TPTString {
    pub fn new<T: Into<Option<String>>>(s: T) -> Self {
        let content = s.into();
        match content {
            Some(content) => {
                if !content.trim().is_empty() {
                    Self(Some(content))
                } else {
                    Self(None)
                }
            }
        None => Self(None)
        }
    }
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for TPTString {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for TPTString {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Some(s.to_string())))
    }

}