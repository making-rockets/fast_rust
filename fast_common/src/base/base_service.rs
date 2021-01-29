use crate::base::base_model::BaseModel;
use rbatis::core::db::DBExecResult;
use rbatis::core::error::Error;
use serde::de::DeserializeOwned;
use actix_web::web::Form;
use std::collections::HashMap;
use std::any::Any;

pub trait BaseService<M> where M: DeserializeOwned {
    type Model: BaseModel;

    async fn save(arg:Form<HashMap<String,Any>>) -> Result<DBExecResult, Error>{

    }

    async fn update(m: M) -> Result<DBExecResult, Error>;
    async fn delete(id: i64) -> Result<DBExecResult, Error>;
    async fn list(m: M) -> Result<DBExecResult, Error>;
    async fn list_page(m: M);
}