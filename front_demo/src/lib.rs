use std::collections::HashMap;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn http_request(url: &str, headers: HashMap<String, String>, body: Option<String>) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(if body.is_some() { "POST" } else { "GET" });
    opts.mode(RequestMode::Cors);
    for (k, v) in headers.iter() {
        opts.headers().set(k, v)?;
    }
    opts.body(body.as_deref());

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp = window.fetch_with_request(&request).await?;

    if !resp.ok() {
        return Err(JsValue::from_str(&format!("HTTP error: {}", resp.status())));
    }

    let text = resp.text().await?;
    Ok(text)
}

pub struct JsHashMap(HashMap<String, String>);

impl WasmDescribe for JsHashMap {
    fn describe() {
        todo!()
    }
}

impl FromWasmAbi for JsHashMap {
    type Abi = JsValue;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        let t: HashMap<String, String> = serde_json::from_str(&js.as_string().unwrap()).unwrap();
        JsHashMap(t)
    }
}
