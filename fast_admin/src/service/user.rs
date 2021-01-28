use fast_common::common::base::RB;
use fast_common::models::domain::user::User;
use fast_common::models::dto::user_dto::UserDTO;
use fast_common::utils::redis_util;
use fast_common::rbatis::core::value::DateTimeNow;
use fast_common::rbatis::core::Result;
use fast_common::rbatis::plugin::page::{PageRequest, Page};
use fast_common::rbatis::core::db::DBExecResult;
use fast_common::rbatis::crud::CRUD;
use fast_common::rbatis::plugin::snowflake::async_snowflake_id;
use chrono::NaiveDateTime;

pub struct UserService {}

impl UserService {
    pub async fn add(arg: &UserDTO) -> Result<DBExecResult> {
        let id = async_snowflake_id().await;
        let user = User {
            id,
            user_name: (*arg.user_name).to_string(),
            age: arg.age,
            create_time: NaiveDateTime::now(),
        };

        let result = RB.save("", &user).await?;
        return Ok(result);
    }

    pub async fn update(user: UserDTO) {
        println!("{:?}", user);
    }

    pub async fn list(arg: &UserDTO) -> Result<Page<User>> {
        let page_req = PageRequest::new(arg.page_num, arg.page_size);
        let wrapper = RB.new_wrapper().check().unwrap();
        let page: Page<User> = RB
            .fetch_page_by_wrapper("", &wrapper, &page_req)
            .await
            .unwrap();
        return Ok(page);
    }
}
