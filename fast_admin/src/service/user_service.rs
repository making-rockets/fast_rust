use fast_common::common::orm_config::RB;
use fast_common::models::domain::user::User;


use chrono::NaiveDateTime;
use rbatis::core::db::DBExecResult;

use rbatis::core::value::DateTimeNow;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::async_snowflake_id;

pub struct UserService {}

impl UserService {
    pub async fn add(mut arg: User) -> Result<DBExecResult> {
        let id = async_snowflake_id().await;

        //arg.id.unwrap() = id as u64;
        let result = RB.save("", &arg).await?;
        return Ok(result);
    }

    pub async fn update(user: User) {
        println!("{:?}", user);
    }

    pub async fn list(arg: User) -> Result<Page<User>> {
        let page_request = PageRequest::new(arg.page_num.unwrap(), arg.page_size.unwrap()); //分页请求，页码，条数
        let wrapper = RB.new_wrapper();
        let page: Result<Page<User>> = RB.fetch_page_by_wrapper("", &wrapper, &page_request).await;
        return page;
    }
}
