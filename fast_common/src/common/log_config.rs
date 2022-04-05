//use actix_web::{middleware::Logger, App};


pub struct Log {

}

impl Log {
    pub fn new() -> Self {
        env_logger::init();
        Self {

        }
    }


}