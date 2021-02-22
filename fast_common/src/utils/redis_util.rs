use redis::AsyncCommands;
use redis_async_pool::deadpool::managed::Pool;
use redis_async_pool::{RedisConnection, RedisConnectionManager, RedisPool};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::Deref;

///缓存服务
pub struct RedisUtil {
    //pub pool:Pool<RedisConnection,RedisError>
}

/*lazy_static! {
    pub static ref CLIETN: redis::Client =
        redis::Client::open(String::from("redis://root:root@localhost:6379")).unwrap();
}*/

// Create a pool of maximum 5 connections, checked on reuse without ttl.

impl RedisUtil {
    pub async fn get_conn() -> ! {
        let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
        let pool = RedisPool::new(RedisConnectionManager::new(client, true, None), 5);
        let x = pool.get().await?;
    }

    pub async fn set_json<T>(&self, k: &String, v: &T) -> Result<String, &str>
    where
        T: Serialize,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err("序列化格式错误");
        }

        let data = self.set_string(&k, data.unwrap().as_str()).await?;
        Ok(data)
    }
    pub async fn get_json<T>(&self, k: &String) -> Result<T, &str>
    where
        T: DeserializeOwned,
    {
        let r = self.get_string(k).await?;
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err("反序列化错误");
        }
        Ok(data.unwrap())
    }
    //TODO 改造redis 工具类

    pub async fn set_string(&self, k: &String, v: &str) -> Result<String, &str> {
        let mut conn = Self::get_conn().await.multiplexed_connection;
        let r: String = redis::cmd("SET")
            .arg(&[k, v])
            .query_async(&mut conn)
            .await
            .unwrap_or(String::new());
        Ok(r)
    }

    pub async fn get_string(&self, k: &str) -> Result<String, &str> {
        let mut conn = Self::get_conn().await.multiplexed_connection;
        let r: String = redis::cmd("GET")
            .arg(&[k])
            .query_async(&mut conn)
            .await
            .unwrap_or(String::new());
        if r.is_empty() {
            return Err("cache data is empty!");
        }
        Ok(r)
    }
}
