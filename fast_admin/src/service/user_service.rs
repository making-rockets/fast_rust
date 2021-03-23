use chrono::{Local, NaiveDateTime};
use fast_common::{common::orm_config::RB, utils::redis_util::{self, RedisUtil}};
use fast_common::models::user::{User, UserLoginVo, UserVo, UserRoleMenuVo};
use fast_common::utils::crypt_util;
use rbatis::core::db::DBExecResult;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::plugin::snowflake::async_snowflake_id;

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
        wrapper = wrapper
            .do_if(false, |wrapper| wrapper.eq("id", &arg.id))
            .do_if(false, |wrapper| wrapper.like_left("user_name", &arg.user_name))
            .do_if(false, |wrapper| wrapper.eq("age", &arg.age))
            .do_if(false, |wrapper| wrapper.gt("create_time", &arg.start_time).and().le("create_time", &arg.end_time));


        let page_request = PageRequest::new(arg.page_num.unwrap_or_else(|| 1), arg.page_size.unwrap_or_else(|| 10));
        let page = RB.fetch_page_by_wrapper("", &wrapper, &page_request).await;
        return page;
    }

    pub async fn login(user_login_vo: UserLoginVo ) -> Result<UserRoleMenuVo> {
        let mut wrapper = RB.new_wrapper();
        if user_login_vo.user_name.is_none() || user_login_vo.password.is_none() || user_login_vo.bar_code.is_none() {
            Err(Error::from("required user_name or password or bar_code"))
        } else {
            let user_name = user_login_vo.user_name.unwrap();
            let user_password = user_login_vo.password.unwrap();
            let bar_code =user_login_vo.bar_code.unwrap();


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

                                Ok( UserRoleMenuVo{
                                    user_id: None,
                                    user_name: None,
                                    role_id: None,
                                    role_name: None,
                                    menus: None
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

   async  fn verify_bar_code(user_name:&String,bar_code:String){
         let redisUtil = RedisUtil::get_redis_util().await;
         let redis_bar_code = redisUtil.get_string(&user_name).await.unwrap();
         match redis_bar_code {
             Value: => {},
         }
         assert_eq!(bar_code,redis_bar_code);
         
        
    }
}

#[test]
fn test() {
    let t = String::from("abc");
    let x = "abc".to_string();
    assert_eq!(t, x);
}
