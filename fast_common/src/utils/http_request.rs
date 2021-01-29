use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest::{Error, Response};
use std::result::Result;

use actix_http::http::HeaderValue;
use reqwest::Url;
use std::cell::RefCell;
use std::collections::hash_map::RandomState;

struct HttpUtil {}

impl HttpUtil {
    async fn post(
        url: &str,
        headers: &mut HeaderMap<HeaderValue>,
        data: &mut HashMap<String, String, RandomState>,
    ) -> Result<Response, Error> {
        let violations = RefCell::new(Vec::new());
        let url = Url::options()
            .syntax_violation_callback(Some(&|v| violations.borrow_mut().push(v)))
            .parse(url)
            .unwrap();

        let client = reqwest::Client::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut data = HashMap::new();
        data.insert("user", "tangjz");
        data.insert("password", "dev-tang.com");
        let result = client.post(url).send().await.unwrap();

        Ok(result)
    }

    async fn get(url: &str) -> Result<Response, Error> {
        let violations = RefCell::new(Vec::new());
        let url = Url::options()
            .syntax_violation_callback(Some(&|v| violations.borrow_mut().push(v)))
            .parse(url)
            .unwrap();
        let response = reqwest::get(url).await?;
        Ok(response)

        //Ok(reqwest::get(url).json::<HashMap<String, String>>().await.unwrap())
    }
}
