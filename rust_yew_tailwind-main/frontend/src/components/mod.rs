#[macro_export]
macro_rules! C {
    ($e:expr) => { classes!(String::from($e)) };
}

pub mod header;