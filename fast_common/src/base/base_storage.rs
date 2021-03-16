use crate::utils::redis_util::RedisUtil;
use std::future::Future;
use async_trait::async_trait;

#[async_trait]
pub trait BaseStorage {
    const PREFIX_KEY: &'static str = "fast:rust:";

    async fn redis_util(&self) -> RedisUtil {
        let util = RedisUtil::get_redis_util().await;
        return util;
    }

    async fn cache_entity<T>(&self, key: &String, t: T);
    async fn get_entity<T>(&self, key: &String) -> T;
}