use async_trait::async_trait;

use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait BaseService: Sync + Send {
    type Model: Serialize + DeserializeOwned;

    fn get_wrapper(arg: &Self::Model);
}
