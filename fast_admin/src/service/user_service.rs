use fast_common::common::orm_config::RB;
use fast_common::models::domain::user::User;
use fast_common::models::domain::user::UserRequest;

use chrono::NaiveDateTime;
use rbatis::core::db::DBExecResult;
use rbatis::core::value::DateTimeNow;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::async_snowflake_id;

pub struct UserService {}

impl UserService {
    pub async fn add(arg: UserRequest) -> Result<DBExecResult> {
        let id = async_snowflake_id().await;
        let user = User {
            id,
            user_name: arg.user_name.unwrap(),
            age: arg.age.unwrap(),
            create_time: NaiveDateTime::now(),
        };

        let result = RB.save("", &user).await?;
        return Ok(result);
    }

    pub async fn update(user: UserRequest) {
        println!("{:?}", user);
    }

    pub async fn list(arg: UserRequest) -> Result<Page<User>> {
        RB.update_batch_by_id()
        let page_req = PageRequest::new(arg.page_num.unwrap(), arg.page_size.unwrap());
        let wrapper = RB.new_wrapper().check().unwrap();
        let page: Result<Page<User>> = RB.fetch_page_by_wrapper("", &wrapper, &page_req).await;
        return page;
    }
}
