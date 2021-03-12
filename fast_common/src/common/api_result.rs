use actix_http::{Response, ResponseBuilder};
use actix_web::{HttpResponse, Responder};

use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};
use actix_web::http::StatusCode;
use actix_web::dev::HttpResponseBuilder;
use actix_http::http::HeaderValue;
use actix_http::http::header::{CONTENT_TYPE, CONTENT_DISPOSITION, ContentType};
use std::fs::File;
use actix_http::error::ResponseError;


#[derive(Debug, Serialize, Clone)]
pub struct Api<T, E> {
    pub code: Option<u16>,
    pub msg: Option<E>,
    pub data: Option<T>,
}


impl<T, E> Api<T, E> where T: Serialize + DeserializeOwned + Clone, E:Serialize+ {
    pub async fn from(result: Result<T, E>) -> Self<T, E> {
        match result {
            Ok(t) => {
                Api { code: Some(StatusCode::OK.as_u16()), msg: None, data: Some(t) }
            }
            Err(  e) => {
                Api { code: Some(StatusCode::BAD_REQUEST.as_u16()), msg: Some(e), data: None }
            }
        }
    }

    pub async fn to_response_of_json(&mut self) -> Response {
        let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap());
        builder.set_header(CONTENT_TYPE, mime::APPLICATION_JSON);
        return builder.body(self.to_string().await);
    }
    pub async fn to_response_of_text(&mut self) -> Response {
        let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap());
        return builder.set_header(CONTENT_TYPE, mime::TEXT_PLAIN_UTF_8).body(self.to_string().await);
    }
    pub async fn to_response_of_html(&mut self) -> Response {
        let mut builder = HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap());
        return builder.set_header(CONTENT_TYPE, mime::TEXT_HTML_UTF_8).body(self.to_string().await);
    }
    pub async fn to_response_of_img(&mut self) -> Response {
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



