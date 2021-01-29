use serde::Serialize;
use short_crypt::ShortCrypt;

const KEY_PR: &'static str = "A391D)@!9sdk#ayS$*#6123#lVB@^?<5"; // key, 32位长度


pub fn encrypt<T: Serialize>(obj: &T) -> Result<String, &'static str> {
    let value = if let Ok(v) = serde_json::to_string(obj) {
        v
    } else {
        return Err("将结构体序列化时出错");
    };
    let sc = ShortCrypt::new(KEY_PR);
    let encrypt_string = sc.encrypt_to_url_component(&value);
    /*let json_data = SwapData {
        data: encrypt_string,
    };*/
    Ok(encrypt_string)
}

pub fn decrypt_string(encrypt_string: &str) -> Result<String, &'static str> {
    let sc = ShortCrypt::new(KEY_PR);
    match sc.decrypt_url_component(encrypt_string) {
        Ok(v) => {
            match String::from_utf8(v) {
                Ok(s) => Ok(s),
                Err(_) => {
                    Err("反解析字符串时出错")
                }
            }
        },
        Err(_) => {
            Err("反解密字符串时出错")
        }
    }
}