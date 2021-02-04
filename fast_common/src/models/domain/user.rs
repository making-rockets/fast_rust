use crate::rbatis;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};


#[crud_enable()]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Option<u64>,
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub create_time: Option<NaiveDateTime>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,

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


