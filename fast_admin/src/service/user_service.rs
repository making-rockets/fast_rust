use chrono::{Local, NaiveDateTime};
use fast_common::{common::orm_config::RB, utils::redis_util::{self, RedisUtil}};
use fast_common::models::user::{User, UserLoginVo, UserVo, UserRoleMenuVo};
use fast_common::utils::crypt_util;
use rbatis::core::db::DBExecResult;
use rbatis::core::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};


use rbatis::Error;
use fast_common::utils::crypt_util::Crypt;
use actix_web::HttpResponse;
use rbatis::wrapper::Wrapper;
use fast_common::base::base_service::BaseService;
use fast_common::common::api_result::{Api, GlobalError};
use crate::service::menu_service::MenuService;
use crate::service::role_service;
use crate::service::role_service::RoleService;
use fast_common::models::role::Role;
use fast_common::utils::redis_util::REDIS_UTIL;

pub struct UserService {}


impl BaseService<User, UserVo> for UserService {
    fn get_wrapper(arg: &UserVo) -> Wrapper {
        let wrapper = RB.new_wrapper();
        wrapper
    }
}


impl UserService {
    pub async fn add(mut user: User) -> Result<DBExecResult> {
        let id = 1 as u64;
        user.id = Some(id);
        let format = "%Y-%m-%d %H:%M:%S";
        user.create_time = Some(NaiveDateTime::parse_from_str(&Local::now().format(format).to_string(), format).unwrap());
        let string = user.password.unwrap_or_else(|| "111111".to_string());
        let result1 = Crypt::encrypt(&string);
        user.password = Some(result1.unwrap());
        let mut wrapper = RB.new_wrapper();
        let result = RB.save(&user, &[]).await?;
        return Ok(result);
    }

    pub async fn update(mut user: User) -> Result<u64> {
        let mut wrapper = RB.new_wrapper();
        let result = RB.update_by_wrapper(&user, wrapper, &[]).await;
        return result;
    }

    pub async fn delete(user: User) -> Result<u64> {
        let mut wrapper = RB.new_wrapper();
        let result = RB.remove_by_wrapper::<User>(wrapper).await;
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
        let page = RB.fetch_page_by_wrapper(wrapper, &page_request).await;
        return page;
    }

    pub async fn login(user_login_vo: UserLoginVo) -> Result<UserRoleMenuVo> {
        let mut wrapper = RB.new_wrapper();
        if user_login_vo.user_name.is_none() || user_login_vo.password.is_none() || user_login_vo.bar_code.is_none() {
            Err(Error::from("required user_name or password or bar_code"))
        } else {
            let user_name = user_login_vo.user_name.unwrap();
            let user_password = user_login_vo.password.unwrap();
            let bar_code = user_login_vo.bar_code.unwrap();
            let result = Self::verify_bar_code(&user_name, bar_code).await;
            if result.is_err() {
                if let Some(e) = result.err() { return Err(Error::from(e)); }
            }

            wrapper = wrapper.eq("user_name", user_name);
            let user_result = RB.fetch_by_wrapper::<User>(wrapper).await;
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
                                //TODO
                                let user_id = user.clone().id.unwrap() as i64;
                                let roles = RoleService::find_role_by_user(user_id).await;
                                if roles.is_err() {
                                    return Err(Error::from("没有角色".to_string()));
                                }

                                let menus = MenuService::find_menus_by_role(roles.unwrap().id.unwrap()).await;

                                Ok(UserRoleMenuVo {
                                    user_id: None,
                                    user_name: None,
                                    access_token: Some(access_token),
                                    role_id: None,
                                    role_name: None,
                                    // menus: Some(menus),
                                    menus: None,
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

    async fn verify_bar_code(user_name: &String, bar_code: String) -> std::result::Result<String, String> {
        let redis_result = REDIS_UTIL.get_string(&user_name).await;
        println!("{:?}", redis_result);

        match redis_result {
            Ok(ret) => {
                println!("{:?},{:?}", &ret, &bar_code);
                if bar_code != (ret) {
                    Err(Error::from("验证码无效").to_string())
                } else {
                    Ok("".to_string())
                }
            }
            Err(err) => { Err("验证码错误".to_string()) }
        }
    }
}

#[test]
fn test() {
    let t = String::from("abc");
    let x = "abc".to_string();
    assert_eq!(t, x);
}
