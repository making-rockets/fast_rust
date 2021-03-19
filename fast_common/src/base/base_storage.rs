use crate::utils::redis_util::RedisUtil;
use std::future::Future;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait BaseStorage: Sync + Send {
    // const PREFIX_KEY: &'static str = "fast:rust:";

    async fn redis_util(&self) -> RedisUtil {
        let util = RedisUtil::get_redis_util().await;
        return util;
    }

    async fn cache_entity<'a, T>(&self, key: &'a String, t: &'a T) -> Result<String, &'a str> where &'a T: Send + Serialize + Sync;
    async fn get_entity<T>(&self, key: &String) where T: Send + Serialize+Sync;
}