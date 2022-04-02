use anyhow::{Ok};
use deadpool_redis::redis::{cmd};
use deadpool_redis::{Config, Connection, Runtime};


use std::string::String;
use redis::{AsyncCommands, ToRedisArgs};
use serde::Serialize;

///缓存服务
#[derive(Debug)]
pub struct RedisUtil;

impl RedisUtil {
    pub async fn get_instance() -> Self {
        Self
    }

    pub async fn get_conn(&self) -> anyhow::Result<Connection> {
        let cfg = Config::from_url("redis://127.0.0.1/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        Ok(pool.get().await?)
    }

    pub async fn set_string(&self, k: &str, v: String) -> anyhow::Result<()> {
        let mut conn = self.get_conn().await?;
        cmd("SET").arg(k).arg(v).query_async(&mut conn).await?;
        Ok(())
    }

    pub async fn get_string(&self, k: &str) -> anyhow::Result<String> {
        let mut conn = self.get_conn().await?;
        Ok(cmd("GET").arg(k).query_async(&mut conn).await?)
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> anyhow::Result<()>
        where
            T: Serialize + ToRedisArgs,
    {
        let result = serde_json::to_string(v)?;
        self.set_string(k, result).await
    }
    pub async fn get_json<T>(&self, k: &str) -> anyhow::Result<T>
        where
            T: serde::de::DeserializeOwned,
    {
        let mut conn = self.get_conn().await?;
        let x = self.get_string(k).await?;
        let result: T = serde_json::from_str(x.as_str())?;
        Ok(result)
    }
}
