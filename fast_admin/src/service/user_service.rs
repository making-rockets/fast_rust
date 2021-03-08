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


use rbatis::Error;
use fast_common::utils::crypt_util::Crypt;

pub struct UserService {}

impl UserService {
    pub async fn add(mut user: User) -> Result<DBExecResult> {
        let id = async_snowflake_id().await as u64;
        user.id = Some(id);
        let format = "%Y-%m-%d %H:%M:%S";
        user.create_time = Some(NaiveDateTime::parse_from_str(&Local::now().format(format).to_string(), format).unwrap());
        let string = user.password.unwrap_or_else(|| "111111".to_string());
        let result1 = Crypt::encrypt(&string);
        user.password = Some(result1.unwrap());
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

    pub async fn login(user_login_vo: UserLoginVo) -> Result<UserLoginVo> {
        let mut wrapper = RB.new_wrapper();
        if user_login_vo.user_name.is_none() || user_login_vo.password.is_none() {
            Err(Error::from("could not found user_name or password"))
        } else {
            let user_name = user_login_vo.user_name.unwrap();
            let user_password = user_login_vo.password.unwrap();

            wrapper = wrapper.eq("user_name", user_name);
            let user_result = RB.fetch_by_wrapper::<User>("", &wrapper).await;
            match user_result {
                Ok(user) => {
                    let password = user.clone().password.unwrap();
                    let password_encrypt_result = Crypt::decrypt_string(&password);

                    match password_encrypt_result {
                        Ok(decrypt) => {
                            let string = format!("{}{}{}", "\"", user_password, "\"");
                            if string == decrypt {
                                //TODO 登录逻辑
                                let claims = crypt_util::Claims::new_default(user.clone().id.unwrap().to_string().as_str());
                                let access_token = claims.default_jwt_token().unwrap();
                                //let redis = RedisUtil::get_redis_util().await;
                                //redis.set_json(&access_token.to_string(), &user.clone()).await;
                                Ok(UserLoginVo {
                                    token: Some(access_token),
                                    user_name: user.clone().user_name,
                                    user_id: None,
                                    password: None,
                                })
                            } else {
                                Err(Error::from("密码错误"))
                            }
                        }
                        Err(err) => { Err(Error::from(format!("解密失败错误:{}", err))) }
                    }
                }
                Err(err) => { Err(Error::from(err.to_string().as_str())) }
            }
        }
    }
}

#[test]
fn test() {
    let t = String::from("abc");
    let x = "abc".to_string();
    assert_eq!(t, x);
}
