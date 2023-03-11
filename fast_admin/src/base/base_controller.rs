use std::fmt::{Debug, Display, format};
use actix_web::web::Form;
use async_trait::async_trait;
use deadpool_postgres::{Manager, Object};

use crate::base::base_model::BaseModel;


#[async_trait]
pub trait BaseController<T> where T: Clone + Debug + Send + Sync  {

}


pub struct Page<T> where T: Clone + Debug + Send + Sync {
    page_number: Option<i64>,
    page_size: Option<i64>,
    data: Option<T>,
    total: Option<i64>,

}