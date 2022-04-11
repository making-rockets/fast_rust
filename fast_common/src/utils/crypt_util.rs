use std::str::from_utf8;
use serde::Serialize;
use serde::Deserialize;
use short_crypt::ShortCrypt;
use chrono::{Local};
use std::string::String;
use crate::models::user::{UserRoleMenuVo, UserVo};
use crate::utils::redis_util::REDIS_UTIL;
use anyhow::{Error, Result};
use futures::future::{err, ok};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::models::menu::MenuVo;
use crate::models::role::RoleVo;


const KEY_PR: &str = "abcdefjhigklmnopqrstuvwxyz1234567890";
const SIGN_KEY: &str = "abcdefghigklmnopqrstuvwxyz1234567890";
// key, 32位长度
const SECRET_KEY: &[u8; 13] = b"a.and.b.and.c";


pub struct Crypt;

impl Crypt {
    pub fn encrypt<T: Serialize + ?Sized>(obj: &T) -> Result<String> {
        let value = serde_json::to_string(obj)?;
        let sc = ShortCrypt::new(KEY_PR);
        let encrypt_string = sc.encrypt_to_url_component(&value);
        Ok(encrypt_string)
    }
    pub fn decrypt_string(encrypt_string: &str) -> Result<String> {
        let sc = ShortCrypt::new(KEY_PR);

        // let result = sc.decrypt(encrypt_string).unwrap();
        //
        // Ok(String::from_utf8(result)?)
        Ok("".to_string())
        // match result {
        //     Ok(vec) => {
        //
        //     }
        //     Err(e) => { anyhow::Error::from(e)}
        // }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u64,
    pub user_name: String,
    pub role: RoleVo,
    pub menus: Vec<MenuVo>,
    express_time: usize,
}


impl Claims {
    pub fn new(user_id: u64, user_name: String, role: RoleVo, menus: Vec<MenuVo>, express_time: usize) -> Self {
        Claims {
            user_id,
            user_name,
            role,
            menus,
            express_time,
        }
    }

    pub fn encode(&self, _sign: &str) -> Result<String> {
        let mut header = Header::default();
        header.kid = Some(SIGN_KEY.to_owned());
        header.alg = Algorithm::HS512;
        let result = jsonwebtoken::encode(&header, &self, &EncodingKey::from_secret(SECRET_KEY))?;
        Ok(result)
    }

    pub fn decode(token: String) -> Result<Self> {
        let result = jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(SECRET_KEY), &Validation::new(Algorithm::HS512))?;
        Ok(result.claims)
    }
    pub fn default_jwt_token(&self) -> Result<String> {
        let result = self.encode(&SIGN_KEY)?;
        return Ok(result);
    }

    pub fn validation_token(access_token: &str) -> Result<()> {
       // let claims = Self::decode(String::from(access_token))?;
        Ok(())
    }


    ///获取用户信息
    async fn get_user_info_by_access_token(access_token: &str) -> Result<UserRoleMenuVo> {
        let x = REDIS_UTIL.get_string(access_token).await?;
        let claims = Self::decode(x)?;
        Ok(UserRoleMenuVo::from(claims))
    }
}


impl From<Claims> for UserRoleMenuVo {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: Some(claims.user_id),
            user_name: Some(claims.user_name),
            access_token: None,
            role_id: Some(claims.role.role_id.unwrap()),
            role_name: Some(claims.role.role_name.unwrap()),
            menus: Some(claims.menus),
        }
    }
}

