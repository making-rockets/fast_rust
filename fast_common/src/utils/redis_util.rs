use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Cmd, Pipeline, RedisFuture, RedisResult, ToRedisArgs, Value};
use serde::de::DeserializeOwned;
use serde::{de::Deserialize, Serialize};

///缓存服务
pub struct RedisUtil {
    pub multiplexed_onnection: MultiplexedConnection,
}

lazy_static! {
    pub static ref CLIETN: redis::Client =
        redis::Client::open(String::from("redis://root:root@localhost:6379")).unwrap();
}

impl RedisUtil {
    pub async fn get_conn() -> RedisUtil {
        let mut multiplexed_onnection =
            CLIETN.get_multiplexed_async_std_connection().await.unwrap();
        RedisUtil {
            multiplexed_onnection,
        }
        // return multiplexed_connection;
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
    pub async fn get_json<T>(&self, k: &str) -> Result<T, &str>
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

    pub async fn set_string(&self, k: &String, v: &str) -> Result<String, &str> {
        let mut conn = Self::get_conn().await.multiplexed_onnection;
        let r: String = redis::cmd("SET")
            .arg(&[k, v])
            .query_async(&mut conn)
            .await
            .unwrap_or(String::new());
        Ok(r)
    }

    pub async fn get_string(&self, k: &str) -> Result<String, &str> {
        let mut conn = Self::get_conn().await.multiplexed_onnection;
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
