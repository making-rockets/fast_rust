use chrono::{Local, NaiveDateTime};
use fast_common::common::orm_config::RB;
use fast_common::models::user::{User, UserLoginVo, UserVo};
use fast_common::utils::crypt_util;
use rbatis::core::db::DBExecResult;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::async_snowflake_id;

use fast_common::utils::redis_util::RedisUtil;
use rbatis::core::value::DateTimeNow;
use rbatis::wrapper::Wrapper;
use rbatis::Error;
use std::borrow::Borrow;
use std::cell::Ref;
use std::ops::Deref;
use std::rc::Rc;

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

        let page_request = PageRequest::new(arg.page_num.unwrap(), arg.page_size.unwrap());
        let page = RB.fetch_page_by_wrapper("", &wrapper, &page_request).await;
        return page;
    }

    pub async fn login(user: UserLoginVo) -> Result<UserLoginVo> {
        let mut wrapper = RB.new_wrapper();
        wrapper = wrapper.eq("user_name", user.user_name.unwrap());
        let x = RB.fetch_by_wrapper::<User>("", &wrapper).await;

        return if x.is_ok() {
            let user_key = format!("user_id:{}", x.clone().unwrap().id.unwrap());
            RedisUtil::get_redis_util()
                .await
                .set_json(&user_key, &x.clone().unwrap())
                .await;
            let user_login_vo = UserLoginVo {
                token: Some(crypt_util::get_uuid()),
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
