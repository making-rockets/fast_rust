use reqwest::header::HeaderMap;
use reqwest::Client;

use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use super::localStorage;

pub async fn run(repo: JsValue) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    opts.body(Some(&repo));

    let url = "http://localhost:3000/admin/user/add_user";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    println!("{:?}", resp_value);

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}

pub async fn request_post<T: Serialize + ?Sized + Clone>(
    url: &str,
    body: &T,
) -> anyhow::Result<String> {
    let mut headers = HeaderMap::new();
    let token = localStorage::get("Authorization").await;
    if token.is_ok() {
        let token = token.unwrap();
        if token.is_some() {
            headers.insert(
                "Authorization",
                format!("Bearer {}", token.unwrap()).parse().unwrap(),
            );
        }
    }
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = Client::builder()
        .default_headers(headers)
        //.user_agent("reqwest client")
        .build()
        .unwrap();

    let result = client.post(url).json(&body).send().await?;
    gloo_console::log!("hello,world");
    let json = &result.json::<String>().await?;
    anyhow::Ok(json.to_owned())
}
