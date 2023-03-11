
use chrono::NaiveDateTime;

pub struct RoleMenu {
    pub role_id: Option<u64>,
    pub role_name: Option<String>,
    pub menu_id: Option<u64>,
    pub menu_name: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub remark: String,
}

pub struct RoleMenuVo {
    pub role_id: Option<u64>,
    pub role_name: Option<String>,

}
