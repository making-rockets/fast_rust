use actix_http::Response;
use actix_web::HttpResponse;
use rbatis::core::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: Option<u32>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> ApiResult<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub async fn from_result(result: &Result<T, Error>) -> Self {
        if result.is_ok() {
            Self {
                code: Some(200),
                msg: None,
                data: result.clone().ok(),
            }
        } else {
            Self {
                code: Some(400),
                msg: Some(result.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub async  fn resp(&self) -> Response {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(self.to_string().await);
    }

    pub async fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
