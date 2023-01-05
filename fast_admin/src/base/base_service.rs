use async_trait::async_trait;

 
use serde::{Serialize, de::DeserializeOwned};

 

#[async_trait]
pub trait BaseService: Sync + Send {
    type Model: Serialize + DeserializeOwned;

    fn get_wrapper(arg: &Self::Model) -> ();

    
}
