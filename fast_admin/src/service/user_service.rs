use chrono::{Local};
use fast_common::common::orm_config::RB;
use fast_common::models::user::{User, UserLoginVo, UserVo};
use fast_common::utils::crypt_util;
use rbatis::core::db::DBExecResult;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::async_snowflake_id;

use fast_common::utils::redis_util::RedisUtil;


use rbatis::Error;

pub struct UserService {}

impl UserService {
    pub async fn add(mut user: User) -> Result<DBExecResult> {
        let id = async_snowflake_id().await as u64;
        user.id = Some(id);
        user.create_time = Some(Local::now().naive_local());
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

        let page_request = PageRequest::new(arg.page_num.unwrap_or_else(|| 1), arg.page_size.unwrap_or_else(|| 10));
        let page = RB.fetch_page_by_wrapper("", &wrapper, &page_request).await;
        return page;
    }

    pub async fn login(user: UserLoginVo) -> Result<UserLoginVo> {
        let mut wrapper = RB.new_wrapper();
        let string = user.user_name.expect("not found user_name ");
        wrapper = wrapper.eq("user_name", string);
        let x = RB.fetch_by_wrapper::<User>("", &wrapper).await;

        return if x.is_ok() {
            let claims = crypt_util::Claims::new(x.clone().unwrap().id.unwrap().to_string().as_str(), "", 0);
            let access_token = claims.default_jwt_token().unwrap();

            let redis = RedisUtil::get_redis_util().await;
            redis.set_json(&access_token.to_string(), &x.clone().expect("expect this is a user object")).await;
            let user_login_vo = UserLoginVo {
                token: Some(access_token),
                user_name: x.clone().unwrap().user_name,
                user_id: None,
                password: None,
            };
            Ok(user_login_vo)
        } else {
            Err(Error::from("用户名或密码错误"))
        };
    }
}
