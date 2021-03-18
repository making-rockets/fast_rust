use fast_common::base::base_storage::BaseStorage;
use async_trait::async_trait;


pub struct UserStorage {}

#[async_trait]
impl BaseStorage for UserStorage {
    async fn cache_entity<T>(&self, t: T) where  T:Send+Sync {}

    async fn get_entity<T>(&self, key: &String) where T:Send+Sync {}
}