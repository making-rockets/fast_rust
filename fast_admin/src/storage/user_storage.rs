use fast_common::base::base_storage::BaseStorage;
use async_trait::async_trait;
use serde::{Serialize, Serializer};

pub struct UserStorage {}

#[async_trait]
impl BaseStorage for UserStorage {
    async fn cache_entity<'a, T>(&self, key: &'a String, t: &'a T) -> Result<String, &'a str> where &'a T: Send + Serialize + Sync {
        let redisUtil = self.redis_util().await;
        redisUtil.set_json(key, &t).await
    }

    async fn get_entity<T>(&self, key: &String) where T: Send + Sync + Serialize {}
}