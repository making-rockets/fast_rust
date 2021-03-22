use redis::{AsyncCommands, RedisError, RedisResult, Value};
use redis_async_pool::deadpool::managed::{Object};
use redis_async_pool::{RedisConnection, RedisConnectionManager, RedisPool};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::string::String;
use std::time::Duration;
use redis::cmd;

///缓存服务
#[derive(Debug)]
pub struct RedisUtil {}

impl RedisUtil {
    pub async fn get_redis_util() -> Self {
        RedisUtil {}
    }

    pub async fn get_conn() -> Object<RedisConnection, RedisError> {
        let pool = RedisPool::new(
            RedisConnectionManager::new(redis::Client::open("redis://localhost:6379").expect("cloud not find redis server"), true, None),
            5,
        );
        println!("装货1111");
        let x = pool.try_get().await.expect("zheshigesha ");
        println!("装货");
        return x;
    }

    pub async fn set_json<T>(&self, k: &String, v: &T) -> Result<String, &str> where T: Serialize, {
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
                    return Err("反序列化错误");
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
    pub async fn set_string_expire(&self, k: &String, v: &String, time: usize) -> RedisResult<Value> {
        let mut conn = Self::get_conn().await;

     conn.set_ex(k.to_owned(), v.to_owned(), time).await
    }


    pub async fn get_string(&self, k: &str) -> RedisResult<Value> {
        let mut conn = Self::get_conn().await;
        conn.get(k).await
    }
}
