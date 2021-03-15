use actix_web::{web, Responder, HttpResponse};
use actix_http::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL};
use serde::{Serialize, Deserialize};

use captcha::filters::{Dots, Noise, Wave};

use image::{ColorType, ImageEncoder, Luma, codecs::{self, png}};
use qrcode::QrCode;
use captcha::Captcha;
use crate::utils::redis_util::RedisUtil;

use rbatis::Error;
use crate::common::api_result::{Api, GlobalError};
use crate::models::user::User;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BarCode {
    user_name: Option<String>,
    url: Option<String>,
}


impl BarCode {
    pub async fn captcha() -> impl Responder {
        let mut captcha = Captcha::new();
        captcha
            .add_chars(4)
            .apply_filter(Noise::new(0.5))
            .apply_filter(Wave::new(2.0, 20.0).horizontal())
            .apply_filter(Wave::new(2.0, 20.0).vertical())
            .view(220, 120)
            .apply_filter(Dots::new(0));
        let mut png = captcha.as_png();
        png = None;
        match png {
            Some(p) => {
                HttpResponse::Ok().set_header(ACCESS_CONTROL_ALLOW_ORIGIN, "")
                    .set_header(CACHE_CONTROL, "no-cache")
                    .content_type(mime::IMAGE_PNG.to_string()).body(p)
            }
            None => {
                let error = "生成验证码错误".to_owned();
                let mut api = Api::from_result(Err::<(), GlobalError>(GlobalError(error))).await;
                let res = api.to_response_of_json().await;
                return res;
            }
        }
    }


    pub async fn qrcode(arg: web::Query<BarCode>) -> impl Responder {
        let code = QrCode::new(arg.into_inner().url.unwrap().as_bytes()).unwrap();
        let image = code.render::<Luma<u8>>().max_dimensions(200, 200).build();
        let mut buffer: Vec<u8> = vec![]; // Generate the image data
        png::PngEncoder::new(&mut buffer)
            .write_image(&image, image.width(), image.height(), ColorType::L8)
            .unwrap();
        HttpResponse::Ok()
            .set_header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .set_header(CACHE_CONTROL, "no-cache")
            .content_type(mime::IMAGE_PNG.to_string())
            .body(buffer)
    }

    pub async fn validate_captcha() {}
}


