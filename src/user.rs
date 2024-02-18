use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::dodo::Dodo;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub age: u8,
    pub begin: Option<DateTime<Local>>,
    pub data: Vec<Dodo>,
}

impl Default for User {
    fn default() -> Self {
        User {
            age: 0,
            begin: None,
            data: vec![],
        }
    }
}