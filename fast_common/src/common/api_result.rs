use actix_http::{Response, ResponseBuilder, ResponseError};
use actix_web::{HttpResponse, Responder};

use serde::de::{DeserializeOwned, Expected};
use serde::{Serialize, Serializer};
use actix_web::http::StatusCode;
use actix_web::dev::HttpResponseBuilder;
use actix_http::http::HeaderValue;
use actix_http::http::header::{CONTENT_TYPE, CONTENT_DISPOSITION, ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL};
use std::fs::File;


use std::fmt::{Formatter, Display};

use serde::__private::de::Content;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct GlobalError(pub String);

impl Display for GlobalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<String> for GlobalError {
    fn from(s: String) -> GlobalError {
        GlobalError(s)
    }
}

impl From<rbatis::core::Error> for GlobalError {
    fn from(e: rbatis::core::Error) -> Self {
        GlobalError(e.to_string())
    }
}

impl std::error::Error for GlobalError {}

impl actix_http::error::ResponseError for GlobalError {}

impl From<actix_http::error::Error> for Api<()> {
    fn from(e: actix_http::error::Error) -> Self {
        Api {
            code: Some(e.as_response_error().status_code().as_u16()),
            msg: Some(GlobalError(e.to_string())),
            data: None,
        }
    }
}


#[derive(Debug, Serialize, Clone)]
pub struct Api<T> {
    pub code: Option<u16>,
    pub msg: Option<GlobalError>,
    pub data: Option<T>,
}


impl<T> Api<T> where T: Serialize + DeserializeOwned + Clone {
    pub async fn from_result(result: Result<T, GlobalError>) -> Self {
        match result {
            Ok(t) => {
                Api { code: Some(StatusCode::OK.as_u16()), msg: None, data: Some(t) }
            }
            Err(e) => {
                Api { code: Some(e.status_code().as_u16()), msg: Some(e), data: None }
            }
        }
    }

    pub async fn from_rbatis_result(result: &rbatis::Result<T>) -> Self {
        match result {
            Ok(t) => {
                Api { code: Some(StatusCode::OK.as_u16()), msg: None, data: Some(t.clone()) }
            }
            Err(e) => {
                Api { code: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()), msg: Some(GlobalError(e.to_string())), data: None }
            }
        }
    }

    pub async fn to_response_of_json(&mut self) -> Response {
        ResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap()).set_header(CONTENT_TYPE, mime::APPLICATION_JSON).body(self.to_string().await)
    }

    pub async fn to_response_of_img(&mut self) -> Response {
        let response = ResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap()).set_header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .set_header(CACHE_CONTROL, "no-cache")
            .content_type(mime::IMAGE_PNG.to_string()).body(self.to_vec_u8().await);
        return response;
    }
    pub async fn to_string(&mut self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
    pub async fn to_vec_u8(&mut self) -> Vec<u8> {
        return serde_json::to_vec(&self.data.clone().unwrap()).unwrap();
    }
}



