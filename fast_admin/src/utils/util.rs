 pub const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn get_current_time() -> String {
    let current_time = chrono::Local::now().format(TIME_FORMAT).to_string();
    return current_time;
}
