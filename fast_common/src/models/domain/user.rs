use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use crate::rbatis;

#[crud_enable]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub age: u64,
    pub create_time: NaiveDateTime,
}
