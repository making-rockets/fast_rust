use log::error;
use log::info;


use redis::{Client, RedisResult};
use serde::de::DeserializeOwned;
use serde::Serialize;
use redis_tang::{Builder, Pool, RedisManager};

use std::result::Result;
use actix_web::web::Data;
use redis::aio::MultiplexedConnection;

///缓存服务
pub struct RedisUtil {
    pub client: Client,
}

impl RedisUtil {
    pub async fn build_pool(num_cpus: usize, redis_url: String) -> Result<Pool<RedisManager>, ()> {
        let mgr = RedisManager::new(redis_url);
        Builder::new()
            .always_check(false)
            .idle_timeout(None)
            .max_lifetime(None)
            .min_idle(num_cpus * 2)
            .max_size(num_cpus * 2)
            .build(mgr)
            .await
            .map_err(|_| ())
    }

    pub async fn get_connection(&self, pool: Data<Pool<RedisManager>>)  {
        let  connection = pool.get().await.ok().unwrap().get_conn();
        redis::cmd("PING")
            .query_async::<_, ()>( connection)
            .await
            .unwrap();
    }
}

