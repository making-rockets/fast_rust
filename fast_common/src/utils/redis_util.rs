use anyhow::{anyhow, Ok};
use deadpool_redis::redis::{RedisResult, Value, cmd};
use deadpool_redis::{Config, Connection, Runtime};
use serde::de::Deserialize;

use std::string::String;
use redis::{AsyncCommands, ToRedisArgs, FromRedisValue};
use serde::Serialize;

///缓存服务
#[derive(Debug)]
pub struct RedisUtil;

impl RedisUtil {
    pub async fn get_conn(&self) -> anyhow::Result<Connection> {
        let cfg = Config::from_url("redis://127.0.0.1/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        Ok(pool.get().await?)
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> anyhow::Result<()>
        where
            T: Serialize + ToRedisArgs,
    {
        let mut conn = self.get_conn().await?;
        cmd("SET").arg(k).arg(&v).query_async(&mut conn).await?;
        Ok(())
    }
    pub async fn get_json<'a, T>(&self, k: &'a str) -> anyhow::Result<T>
        where
            T: Deserialize<'a> + serde::Serialize + FromRedisValue,
    {
        let mut conn = self.get_conn().await?;
        let result = cmd("GET").arg(k).query_async(&mut conn).await?;


        let result: T = serde_json::from_str::<T>(result)?;
        Ok(result)
    }
}
