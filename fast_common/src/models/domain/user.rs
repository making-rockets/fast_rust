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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserRequest {
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UserResponse {
    pub id: i64,
    pub user_name: String,
    pub age: u64,
    pub create_time: NaiveDateTime,
}

