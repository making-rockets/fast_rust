use crate::rbatis;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

#[crud_enable]
#[derive(Debug, Serialize, DeserializeOwned, Clone, PartialEq)]
pub struct Role {
    pub id: Option<u64>,
    pub role_name: Option<String>,
    pub index_no: Option<i32>,
    pub remark: Option<String>,
    pub state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, DeserializeOwned, Clone, PartialEq)]
pub struct RoleVo {
    pub role_id: Option<u64>,
    pub role_name: Option<String>,
    pub index_no: Option<i32>,
    pub remark: Option<String>,
    pub state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

