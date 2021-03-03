use redis::{AsyncCommands, RedisError, RedisResult, Value};
use redis_async_pool::deadpool::managed::{Object, Pool};
use redis_async_pool::{RedisConnection, RedisConnectionManager, RedisPool};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::Deref;
use std::string::String;
use std::fmt::Write;
use futures::future::ok;

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
    pub async fn get_json<T>(&self, k: &String) -> Result<T, &str> where T: DeserializeOwned, {
        let value = self.get_string(k).await.unwrap();
        match value {
            Value::Data(data) => {
                let result: serde_json::Result<T> = serde_json::from_slice::<T>(&data);
                if result.is_err() {
                   return  Err("反序列化错误");
                }
                Ok(result.unwrap())
            }
            // Value::Bulk(bluk) => { println!("bluk -> {:?}");return Ok(bluk) }
            _ => {
                println!("{:?}", value);
                Err("没有找到数据")
            }
        }
    }
    //TODO 改造redis 工具类

    pub async fn set_string(&self, k: &String, v: &String) -> RedisResult<String> {
        let mut conn = Self::get_conn().await;
        let result = conn.set(k, v).await;
        return result;
    }


    pub async fn get_string(&self, k: &str) -> RedisResult<Value> {
        let mut conn = Self::get_conn().await;
        conn.get(k).await
    }
}
