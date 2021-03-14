use actix_http::{Response, ResponseBuilder, ResponseError};
use actix_web::{HttpResponse, Responder};

use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use actix_web::http::StatusCode;
use actix_web::dev::HttpResponseBuilder;
use actix_http::http::HeaderValue;
use actix_http::http::header::{CONTENT_TYPE, CONTENT_DISPOSITION, ContentType};
use std::fs::File;


use std::fmt::{Formatter, Display};
use rbatis::Error;

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

impl From<actix_http::error::Error> for GlobalError {
    fn from(e: actix_http::error::Error) -> Self {
        GlobalError(e.to_string())
    }
}


impl ResponseError for GlobalError {}

impl std::error::Error for GlobalError {}

#[derive(Debug, Serialize, Clone)]
pub struct Api<T> {
    pub code: Option<u16>,
    pub msg: Option<GlobalError>,
    pub data: Option<T>,
}


impl<T> Api<T> where T: Serialize + DeserializeOwned + Clone {
    pub async fn from(result: Result<T, GlobalError>) -> Self {
        match result {
            Ok(t) => {
                Api { code: Some(StatusCode::OK.as_u16()), msg: None, data: Some(t) }
            }
            Err(e) => {
                Api { code: Some(e.status_code().as_u16()), msg: Some(e), data: None }
            }
        }
    }

    pub async fn to_response_of_json(&mut self) -> HttpResponse {
        let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap());
        builder.set_header(CONTENT_TYPE, mime::APPLICATION_JSON);
        let response = builder.body(self.to_string().await);
        return response;
    }
    pub async fn to_response_of_text(&mut self) -> HttpResponse {
        let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap());
        return builder.set_header(CONTENT_TYPE, mime::TEXT_PLAIN_UTF_8).body(self.to_string().await);
    }
    pub async fn to_response_of_html(&mut self) -> HttpResponse {
        let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap());
        return builder.set_header(CONTENT_TYPE, mime::TEXT_HTML_UTF_8).body(self.to_string().await);
    }
    pub async fn to_response_of_img(&mut self) -> HttpResponse {
        let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap());
        return builder.set_header(CONTENT_TYPE, mime::IMAGE_STAR).body(self.to_string().await);
    }
    pub async fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}

#[test]
fn test() {
    println!("{}", StatusCode::OK.as_u16());
}



