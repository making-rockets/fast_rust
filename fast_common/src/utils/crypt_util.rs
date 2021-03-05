use serde::Serialize;
use serde::Deserialize;
use short_crypt::ShortCrypt;
use uuid::Uuid;
use jsonwebtoken::{Header, Algorithm, EncodingKey, DecodingKey, Validation, TokenData};
use jsonwebtoken::errors::{Error, ErrorKind};

const KEY_PR: &'static str = "abcdefjhigklmnopqrstuvwxyz"; // key, 32位长度

pub fn encrypt<T: Serialize>(obj: &T) -> Result<String, &'static str> {
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Claims {
    fn new(sub: &str, company: &str, exp: usize) -> Self {
        Claims { sub: sub.to_string(), company: company.to_owned(), exp }
    }

    fn encode(&self, sign_key: &str) -> Result<String, &str> {
        let mut header = Header::default();
        header.kid = Some(sign_key.to_owned());
        header.alg = Algorithm::HS512;
        match jsonwebtoken::encode(&header, &self, &EncodingKey::from_secret(secret_key)) {
            Ok(token) => { Ok(token) }
            Err(_) => { Err("生成token失败") }
        }
    }

    fn decode(&self, token: &String) -> Result<Self, &str> {
        let result = jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(secret_key), &Validation::new(Algorithm::HS512));
        match result {
            Ok(tokenData) => { Ok(tokenData.claims) }
            Err(err) => match err.kind() {
                ErrorKind::InvalidToken => { Err("无效jwt token") }
                _ => panic!()
            }
        }
    }
}

impl Default for Claims {
    fn default() -> Self {
        Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned(), exp: 10000000000 }
    }
}

const secret_key: &[u8; 13] = b"a.and.b.and.c";


pub fn get_uuid() -> Uuid {
    let uuid = Uuid::new_v4();
    return uuid;
}

#[test]
pub fn mani() {
    let claims = Claims::new("1", "2", 10000);
    let result = claims.encode("2");
    println!("{:?}", result);
}

