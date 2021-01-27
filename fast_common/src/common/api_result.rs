use actix_http::Response;
use actix_web::HttpResponse;
use rbatis::core::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResult<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(result: &Result<T, Error>) -> Self {
        if result.is_ok() {
            Self {
                code: Some("SUCCESS".to_string()),
                msg: None,
                data: result.clone().ok(),
            }
        } else {
            Self {
                code: Some("FAIL".to_string()),
                msg: Some(result.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn resp(&self) -> Response {
        return HttpResponse::Ok()
            .content_type("json")
            .body(self.to_string());
    }

    pub fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
