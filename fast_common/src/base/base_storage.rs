use crate::utils::redis_util::RedisUtil;
use std::future::Future;

pub trait BaseStorage {

    const PREFIX_KEY: &'static str = "fast:rust:";

    fn redis_util(&self) -> Box<Future<Output=RedisUtil>> {
        let util = RedisUtil::get_redis_util();
        return Box::new(util);
    }

    fn cache_entity<T>(&self, key: &String, t: T);
    fn get_entity<T>(&self, key: &String) -> T;
}