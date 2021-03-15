use serde::Serialize;
use serde::Deserialize;
use short_crypt::ShortCrypt;
use chrono::{Local, Timelike};
use jsonwebtoken::{Header, Algorithm, EncodingKey, DecodingKey, Validation};
use std::string::String;

const KEY_PR: &'static str = "abcdefjhigklmnopqrstuvwxyz1234567890";
const SIGN_KEY: &str = "abcdefghigklmnopqrstuvwxyz1234567890";
// key, 32位长度
const SECRET_KEY: &[u8; 13] = b"a.and.b.and.c";


pub struct Crypt;

impl Crypt {
    pub fn encrypt<T: Serialize + ?Sized>(obj: &T) -> Result<String, &'static str> {
        let value = if let Ok(v) = serde_json::to_string(obj) {
            v
        } else {
            return Err("将结构体序列化时出错");
        };
        let sc = ShortCrypt::new(KEY_PR);
        let encrypt_string = sc.encrypt_to_url_component(&value);
        Ok(encrypt_string)
    }
    pub fn decrypt_string(encrypt_string: &str) -> Result<String, &'static str> {
        let sc = ShortCrypt::new(KEY_PR);
        match sc.decrypt_url_component(encrypt_string) {
            Ok(v) => match String::from_utf8(v) {
                Ok(s) => Ok(s),
                Err(_) => Err("反解析字符串时出错"),
            },
            Err(_) => Err("反解密字符串时出错"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: &str, company: &str, exp: usize) -> Self {
        Claims { sub: sub.to_string(), company: company.to_owned(), exp }
    }
    pub fn new_default(sub: &str) -> Self {
        Claims {
            sub: sub.to_owned(),
            company: "bxb".to_string(),
            exp: (Local::now().timestamp() + 120) as usize,
        }
    }

    pub fn encode(&self, sign: &str) -> Result<String, String> {
        let mut header = Header::default();
        header.kid = Some(SIGN_KEY.to_owned());
        header.alg = Algorithm::HS512;
        match jsonwebtoken::encode(&header, &self, &EncodingKey::from_secret(SECRET_KEY)) {
            Ok(token) => { Ok(token) }
            Err(err) => { Err(err.to_string()) }
        }
    }

    pub fn decode(token: &String) -> Result<Self, String> {
        let result = jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(SECRET_KEY), &Validation::new(Algorithm::HS512));
        match result {
            Ok(tokenData) => { Ok(tokenData.claims) }
            Err(err) => { Err(err.to_string()) }
        }
    }
    pub fn default_jwt_token(&self) -> Result<String, String> {
        let result = self.encode(&SIGN_KEY);
        return result;
    }

    pub fn validation_token(token: &String) -> Result<(), String> {
        let result = Self::decode(token);
        match result {
            Ok(claims) => { Ok(()) }
            Err(err) => { Err(err) }
        }
    }
}


impl Default for Claims {
    fn default() -> Self {
        Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned(), exp: 10000000000 }
    }
}


#[test]
fn test() {
    let i = Local::now().timestamp();
    println!("{}", i as usize)
}
