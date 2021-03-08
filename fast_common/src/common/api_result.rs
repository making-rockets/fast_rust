use actix_http::Response;
use actix_web::{HttpResponse, Responder};
use rbatis::core::Error;
use serde::de::DeserializeOwned;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResult<T> {
    pub code: Option<u32>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResult<T> where T: Serialize + DeserializeOwned + Clone {
    pub async fn from_result(result: &Result<T, Error>) -> ApiResult<T> {
        if result.is_ok() {
            Self { code: Some(200), msg: None, data: result.clone().ok() }
        } else {
            match result.clone().err() {
                None => { Self { code: Some(500), msg: Some("服务器内部错误，请联系管理员".to_string()), data: None } }
                Some(e) => { Self { code: Some(400), msg: Some(e.to_string()), data: None } }
            }
        }
    }

    pub async   fn from_error(e: &Error) -> ApiResult<()> {
        let result = ApiResult {
            code: Some(400),
            msg: Some(e.to_string()),
            data: None,
        };
        return result;
    }

    pub async fn resp(&self) -> Response {
        return HttpResponse::Ok().content_type("application/json").body(self.to_string().await);
    }

    pub async fn resp_to_img(&self) -> Response {
        return HttpResponse::Ok().content_type("image/png").body(self.to_string().await);
    }
    pub async fn resp_to_text(&self) -> Response {
        return HttpResponse::Ok().content_type("text/plain").body(self.to_string().await);
    }

    pub async fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}

#[test]
fn test() {}
