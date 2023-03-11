use std::fmt::Formatter;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::base::base_controller::BaseController;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Menu {
    pub id: Option<u64>,
    pub menu_name: Option<String>,
    pub parent_id: Option<u64>,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub index_no: Option<i32>,
    pub remark: Option<String>,
    pub state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}

