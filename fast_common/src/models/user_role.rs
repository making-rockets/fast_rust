use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, DeserializeOwned, Serialize)]
pub struct UserRole {
    id: Option<u64>,
    user_name: Option<String>,
    user_id: Option<u64>,
    role_id: Option<u64>,
    role_name: Option<String>,
}
