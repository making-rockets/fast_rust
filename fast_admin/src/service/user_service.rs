use fast_common::common::orm_config::RB;
use fast_common::models::domain::user::{User, UserResponse};
use fast_common::models::domain::user::UserRequest;

use chrono::NaiveDateTime;
use rbatis::core::db::DBExecResult;

use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::async_snowflake_id;
use rbatis::core::value::DateTimeNow;


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

        let pageRequest = PageRequest::new(arg.page_num.unwrap(), arg.page_size.unwrap());//分页请求，页码，条数
        let wrapper = RB.new_wrapper();
        let page: Result<Page<User>> = RB.fetch_page_by_wrapper("", &wrapper, &pageRequest).await;
        return page;
    }
}
