use anyhow::{Ok};
use deadpool_redis::redis::{cmd};
use deadpool_redis::{Config, Connection, Pool, Runtime};


use std::string::String;
use redis::{Cmd, ToRedisArgs};
use serde::Serialize;

use lazy_static::lazy_static;

lazy_static! {
  pub static ref REDIS_UTIL:RedisUtil=  RedisUtil::get_instance();

}
pub struct RedisUtil {
    pool: Pool,
}

impl RedisUtil {
    fn get_instance() -> RedisUtil {
        let cfg = Config::from_url("redis://39.101.69.31/");
        Self { pool: cfg.create_pool(Some(Runtime::Tokio1)).unwrap() }
    }

    pub async fn get_conn() -> anyhow::Result<Connection> {
        Ok(REDIS_UTIL.pool.get().await?)
    }

    pub async fn set_string(&self, k: &str, v: String) -> anyhow::Result<()> {
        let mut conn = RedisUtil::get_conn().await?;
        cmd("SET").arg(k).arg(v).query_async(&mut conn).await?;
        Ok(())
    }

    pub async fn set_by_nx(&self, k: &str, v: String) -> anyhow::Result<bool> {
        let mut conn = RedisUtil::get_conn().await?;
        let cmd: bool = Cmd::set_nx(k, v).query_async(&mut conn).await?;
        Ok(cmd)
    }

    pub async fn get_string(&self, k: &str) -> anyhow::Result<String> {
        let mut conn = RedisUtil::get_conn().await?;
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
        let x = self.get_string(k).await?;
        let result: T = serde_json::from_str(x.as_str())?;
        Ok(result)
    }
}


pub mod test {
    use deadpool_redis::{Config, Runtime};
    use redis::cmd;

    #[tokio::test]
    async fn main() {
        let mut cfg = Config::from_url("redis://39.101.69.31/");
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        {
            let mut conn = pool.get().await.unwrap();
            cmd("SET")
                .arg(&["deadpool/test_key", "42"])
                .query_async::<_, ()>(&mut conn)
                .await.unwrap();
        }
        {
            let mut conn = pool.get().await.unwrap();
            let value: String = cmd("GET")
                .arg(&["deadpool/test_key"])
                .query_async(&mut conn)
                .await.unwrap();
            assert_eq!(value, "42".to_string());
        }
    }
}
