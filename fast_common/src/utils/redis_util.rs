use log::error;
use log::info;


use redis::{Client, RedisResult};
use serde::de::DeserializeOwned;
use serde::Serialize;
use redis_tang::{Builder, Pool, RedisManager};

use std::result::Result;
use actix_web::web::Data;
use actix::web::crate::HttpRequsetPool;
use redis::aio::MultiplexedConnection;
use actix_web::HttpRequest;

///缓存服务
pub struct RedisUtil {

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

    pub async fn set(&self,key:String,value:String)  {
        let req:HttpRequest = HttpRequestPool::get_request().unwrap();
        let pool = req.app_data::<Pool<RedisManager>>().unwrap();
        let mut connection = pool.get().await.unwrap().clone();
        redis::cmd("PING")
            .query_async::<_, ()>( &mut connection)
            .await
            .unwrap();
    }

}

