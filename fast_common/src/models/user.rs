use crate::rbatis;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[crud_enable]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Option<u64>,
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserVo {
    pub id: Option<i64>,
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserLoginVo {
    pub token: Option<Uuid>,
    pub user_name: Option<String>,
    pub user_id: Option<u64>,
    pub password: Option<String>,
}
