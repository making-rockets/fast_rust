use log::error;
use log::info;
use async_once::AsyncOnce;

use redis::{Client, RedisResult, FromRedisValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use redis_tang::{Builder, Pool, RedisManager, RedisPoolError};

use std::result::Result;
use actix_web::web::Data;

use redis::aio::MultiplexedConnection;
use actix_web::HttpRequest;
use std::sync::Mutex;


///缓存服务
pub struct RedisUtil {}

#[derive(Clone)]
pub struct RedisWrapper(Pool<RedisManager>);



lazy_static! {
    pub static ref pool:AsyncOnce<Pool<RedisManager>> = AsyncOnce::new(async  {
       RedisUtil::pool_builder(1,String::from("redis://localhost:6379")).await.unwrap()
    });
}

impl RedisUtil {

    pub async  fn pool_builder(num_cpus: usize, redis_url: impl redis::IntoConnectionInfo) -> Result<Pool<RedisManager>, RedisPoolError> {
        let mgr = RedisManager::new(redis_url);
        let build = Builder::new().always_check(false).idle_timeout(None).max_lifetime(None).min_idle(num_cpus * 2)
            .max_size(num_cpus * 2).build(mgr).await;
        return build;
    }

    pub async fn set(key: String, value: String) -> RedisResult<String> {
       let mut client = pool.get().await .get().await.unwrap().clone();
        redis::cmd("SET").arg(key).arg(value)
            .query_async::<MultiplexedConnection, String>(  &mut client)
            .await
    }
}
