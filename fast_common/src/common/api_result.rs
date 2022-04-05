use actix_web::http::header;
use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder};
use actix_web::{HttpResponseBuilder, ResponseError};

use serde::de::DeserializeOwned;
use serde::Serialize;

use std::fmt::{Display, Formatter};
use actix_http::body::{BoxBody, MessageBody};
use serde_json::to_string;

#[derive(Debug, Serialize, Clone)]
pub struct GlobalError(pub String);


impl Display for GlobalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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

impl ResponseError for GlobalError {}


impl From<actix_web::error::Error> for Api<()> {
    fn from(e: actix_web::error::Error) -> Self {
        Api {
            code: Some(e.error_response().status().as_u16()),
            msg: Some(GlobalError(e.to_string())),
            data: None,
        }
    }
}




#[derive(Debug, Serialize,Clone)]
pub struct Api<T> where T:Serialize  {
    pub code: Option<u16>,
    pub msg: Option<GlobalError>,
    pub data: Option<T>,
}


// impl<T> Responder for Api<T> where T: Serialize + DeserializeOwned + Clone, {
//     type Body = BoxBody;
//     fn respond_to(mut self, req: &HttpRequest) -> HttpResponse<Self::Body> {
//         self.to_response_of_json().await
//     }
// }


impl<T> Api<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    pub async fn from_result(result: Result<T, GlobalError>) -> Self {
        match result {
            Ok(t) => Api {
                code: Some( StatusCode::OK.as_u16()),
                msg: None,
                data: Some(t),
            },
            Err(e) => Api {
                code: Some(e.status_code().as_u16()),
                msg: Some(e),
                data: None,
            },
        }
    }

    pub async fn from_rbatis_result(result: rbatis::Result<T>) -> Self {
        match result {
            Ok(t) => Api {
                code: Some(StatusCode::OK.as_u16()),
                msg: None,
                data: Some(t),
            },
            Err(e) => Api {
                code: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
                msg: Some(GlobalError(e.to_string())),
                data: None,
            },
        }
    }

    pub async fn to_response_of_json(&mut self) -> HttpResponse {


        HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap())
            //.insert_header(header::ACCESS_CONTROL_ALLOW_METHODS.as_ref())
            .content_type(header::ContentType(mime::APPLICATION_JSON))
            .insert_header(header::AcceptEncoding(vec![
                "gzip".parse().unwrap(),
                "br".parse().unwrap(),
            ]))
            .body(self.to_string().await)
    }

    pub async fn to_response_of_img(&mut self) -> HttpResponse {
        HttpResponseBuilder::new(StatusCode::from_u16(self.code.unwrap()).unwrap())
            .content_type(mime::IMAGE_JPEG.to_string())
            .insert_header(("cache_control", "no-cache"))
            .body(self.to_vec_u8().await)
    }
    pub async fn to_string(&mut self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
    pub async fn to_vec_u8(&mut self) -> Vec<u8> {
        return serde_json::to_vec(&self.data.clone().unwrap()).unwrap();
    }
}
