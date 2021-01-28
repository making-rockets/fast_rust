use std::collections::HashMap;
use serde_json::Value;
use reqwest::header::HeaderMap;
use std::result::Result;
use reqwest::{Response, Error};
use reqwest::RequestBuilder;
use std::collections::hash_map::RandomState;
use actix_http::http::HeaderValue;

trait HttpRequest<K, V> {
    async fn post(&url: &str, headers: &mut HeaderMap, data: &mut HashMap<K, V>) -> Result<HashMap<String, String>, reqwest::Error>;
    fn get(&url: &str) -> Result<HashMap<String, String>, reqwest::Error>;
}

struct HttpUtil {}

impl<K, V> HttpRequest<K, V> for HttpUtil {
    async fn post(&url: &str, mut headers: HeaderMap<HeaderValue>, data: &mut HashMap<K, V, RandomState>) -> Result<HashMap<String, String, RandomState>, Error> {
        let client = reqwest::Client::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut data = HashMap::new();
        data.insert("user", "tangjz");
        data.insert("password", "dev-tang.com");

        Ok(client.post(url.into()).headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
    }

    fn get(&url: &str) -> Result<HashMap<String, String, RandomState>, Error> {
        Ok(reqwest::get(url.into()).await?.json::<HashMap<String, String>>().await?)
    }
}



