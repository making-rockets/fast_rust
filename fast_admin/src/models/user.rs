use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::menu::MenuVo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub create_time: Option<NaiveDateTime>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserVo {
    pub id: Option<i64>,
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub create_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLoginVo {
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub bar_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRoleMenuVo {
    pub user_id: Option<u64>,
    pub user_name: Option<String>,
    pub access_token: Option<String>,
    pub role_id: Option<u64>,
    pub role_name: Option<String>,
    pub menus: Option<Vec<MenuVo>>,
}
