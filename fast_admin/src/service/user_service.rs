use fast_common::common::orm_config::RB;
use fast_common::models::user::{User, UserVo};

use chrono::NaiveDateTime;
use rbatis::core::db::DBExecResult;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::async_snowflake_id;

use rbatis::core::value::DateTimeNow;

pub struct UserService {}

impl UserService {
    pub async fn add(mut user: User) -> Result<DBExecResult> {
        let id = async_snowflake_id().await as u64;
        user.id = Some(id);
        user.create_time = Some(NaiveDateTime::now());
        let result = RB.save("", &user).await?;
        return Ok(result);
    }

    pub async fn update(mut user: User) -> Result<u64> {
        let result = RB.update_by_id("", &mut user).await;
        return result;
    }

    pub async fn delete(user: User) -> Result<u64> {
        let result = RB.remove_by_id::<User>("", &user.id.unwrap()).await;
        return result;
    }

    pub async fn list(arg: UserVo) -> Result<Page<User>> {
        let mut wrapper = RB.new_wrapper();
        if arg.id.is_some() {
            wrapper = wrapper.eq("id", &arg.id);
        }
        if arg.user_name.is_some() {
            wrapper = wrapper.like("%user_name%", &arg.user_name);
        }
        if arg.age.is_some() {
            wrapper = wrapper.eq("age", &arg.user_name);
        }
        if arg.create_time.is_some() {
            wrapper = wrapper.gt("create_time", arg.create_time);
        }

        let page_request = PageRequest::new(arg.page_num.unwrap(), arg.page_size.unwrap());
        let page = RB.fetch_page_by_wrapper("", &wrapper, &page_request).await;

        return page;
    }
}
