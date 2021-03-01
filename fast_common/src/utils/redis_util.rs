use redis::{AsyncCommands, RedisError, RedisResult};
use redis_async_pool::deadpool::managed::{Object, Pool};
use redis_async_pool::{RedisConnection, RedisConnectionManager, RedisPool};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::Deref;

///缓存服务
pub struct RedisUtil {
    pub pool: Object<RedisConnection, RedisError>
}

/*lazy_static! {
    pub static ref CLIETN: redis::Client =
        redis::Client::open(String::from("redis://root:root@localhost:6379")).unwrap();
}*/

// Create a pool of maximum 5 connections, checked on reuse without ttl.

impl RedisUtil {
    pub async fn get_redis_util() -> Self {
        let object = Self::get_conn().await;
        RedisUtil { pool: object }
    }

    pub async fn get_conn() -> Object<RedisConnection, RedisError> {
        let pool = RedisPool::new(
            RedisConnectionManager::new(redis::Client::open("redis://localhost:6379").expect("cloud not find redis server"), true, None),
            5,
        ).get().await.expect("cloud not create a redis pool ");
        return pool;
    }

    pub async fn set_json<T>(&self, k: &String, v: &T) -> Result<String, &str>
        where
            T: Serialize,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err("序列化格式错误");
        }

        let data = self.set_string(&k, &data.unwrap()).await.unwrap();
        Ok(data)
    }
    pub async fn get_json<T>(&self, k: &String) -> Result<T, &str>
        where
            T: DeserializeOwned,
    {
        let r = self.get_string(k).await.unwrap();
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err("反序列化错误");
        }
        Ok(data.unwrap())
    }
    //TODO 改造redis 工具类

    pub async fn set_string(&self, k: &String, v: &String) -> RedisResult<String> {
        let mut conn = Self::get_conn().await;
        let result = conn.set(k, v).await;
        return result;
    }

    pub async fn get_string(&self, k: &str) -> RedisResult<String> {
        let mut conn = Self::get_conn().await;
        let pin: RedisResult<String> = conn.get(k).await;
        return pin;
    }
}
