#[macro_export]
macro_rules! C {
    ($e:expr) => { classes!(String::from($e)) };
}

pub mod header;
pub mod api_result;