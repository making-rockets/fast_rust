use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDTO {
    pub user_name: String,
    pub age: u64,
    pub page_num: u64,
    pub page_size: u64,
}
