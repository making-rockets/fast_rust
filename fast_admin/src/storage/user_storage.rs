use fast_common::base::base_storage::BaseStorage;

pub struct UserStorage {}

impl BaseStorage for UserStorage {
    fn cache_entity<T>(&self, key: &String, t: T) {
         async{
             let x = self.redis_util().await;
         }
    }

    fn get_entity<T>(&self, key: &String) -> T {
        unimplemented!()
    }
}