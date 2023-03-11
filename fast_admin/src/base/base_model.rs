pub struct BaseModel {
    page_num: Option<i64>,
    page_size: Option<i64>,
    create_time: std::time::SystemTime,
    update_time: std::time::SystemTime,
}
