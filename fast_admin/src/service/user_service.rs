use std::borrow::{Borrow, BorrowMut};
use actix_web::error::ErrorBadRequest;
use chrono::{Local, NaiveDateTime};
use fast_common::{common::orm_config::RB, utils::redis_util::{self, RedisUtil}};
use fast_common::models::user::{User, UserLoginVo, UserVo, UserRoleMenuVo};
use fast_common::utils::crypt_util;
use rbatis::core::db::DBExecResult;

use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use anyhow::{anyhow, Error, Result};

use fast_common::utils::crypt_util::Crypt;
use actix_web::HttpResponse;
use rbatis::value::DateTimeNow;
use rbatis::wrapper::Wrapper;
use crypt_util::Claims;
use fast_common::base::base_service::BaseService;
use fast_common::common::api_result::{Api, GlobalError};
use crate::service::menu_service::MenuService;
use crate::service::role_service;
use crate::service::role_service::RoleService;
use fast_common::models::role::Role;
use fast_common::utils::redis_util::REDIS_UTIL;

pub struct UserService {}


impl BaseService for UserService {
    type Model = User;

    fn get_wrapper(arg: &Self::Model) -> Wrapper {
        RB.new_wrapper_table::<Self::Model>()
    }
}

impl UserService {
    pub async fn add(mut user: User) -> anyhow::Result<DBExecResult> {
        user.create_time = Some(NaiveDateTime::now());
        let string = user.password.unwrap_or_else(|| "111111".to_string());
        user.password = Some(Crypt::encrypt(&string)?);
        let result = Self::save(user).await;
        result
    }

    pub async fn update(user: User) -> anyhow::Result<u64> {
        let wrapper = Self::get_wrapper(&user);
        let result = RB.update_by_wrapper(&user, wrapper, &[]).await?;
        Ok(result)
    }

    pub async fn delete(user: User) -> Result<u64> {
        let wrapper = Self::get_wrapper(&user);
        let result = RB.remove_by_wrapper::<User>(wrapper).await?;
        Ok(result)
    }

    pub async fn list(arg: UserVo) -> Result<Page<User>> {
        let mut wrapper = RB.new_wrapper();
        wrapper = wrapper
            .do_if(false, |wrapper| wrapper.eq("id", &arg.id))
            .do_if(false, |wrapper| wrapper.like_left("user_name", &arg.user_name))
            .do_if(false, |wrapper| wrapper.eq("age", &arg.age))
            .do_if(false, |wrapper| wrapper.gt("create_time", &arg.start_time).and().le("create_time", &arg.end_time));


        let page_request = PageRequest::new(arg.page_num.unwrap_or(1), arg.page_size.unwrap_or(10));
        let page: Page<User> = RB.fetch_page_by_wrapper(wrapper, &page_request).await?;
        Ok(page)
    }

    pub async fn login(user_login_vo: UserLoginVo) -> Result<UserRoleMenuVo> {
        let mut wrapper = RB.new_wrapper();
        if user_login_vo.user_name.is_none() || user_login_vo.password.is_none() || user_login_vo.bar_code.is_none() {
            let t = anyhow::Error::msg("params is failed");
            return Err(t);
        } else {
            let user_name = user_login_vo.user_name.unwrap();
            let user_password = user_login_vo.password.unwrap();
            let bar_code = user_login_vo.bar_code.unwrap();
            let result = Self::verify_bar_code(&user_name, bar_code).await;
            if result.is_err() {
                return Err(anyhow!(" verify bar_code is failed" ));
            }

            wrapper = wrapper.eq("user_name", user_name);
            let user_result = RB.fetch_by_wrapper::<User>(wrapper).await;


            match user_result {
                Ok(u) => {
                    if u.password.unwrap().eq(&user_password) {
                        Ok(UserRoleMenuVo {
                            user_id: (u.id),
                            user_name: (u.user_name),
                            access_token: None,
                            role_id: None,
                            role_name: None,
                            menus: None,
                        })
                    } else {
                        Err(anyhow::Error::msg("验证码错误"))
                    }
                }
                Err(e) => { Err(anyhow::Error::msg(e)) }
            }
        }
    }

    async fn verify_bar_code(user_name: &String, bar_code: String) -> Result<String> {
        let redis_result = REDIS_UTIL.get_string(&user_name).await;
        println!("{:?}", redis_result);

        match redis_result {
            Ok(ret) => {
                println!("{:?},{:?}", &ret, &bar_code);
                if bar_code != (ret) {
                    Err(anyhow::Error::msg("无效的验证码"))
                } else {
                    Ok("".to_string())
                }
            }
            Err(err) => { Err(anyhow::Error::msg("验证码错误")) }
        }
    }
}

#[test]
fn test() {
    let t = String::from("abc");
    let x = "abc".to_string();
    assert_eq!(t, x);
}


