use web_sys::Storage;

pub async fn put(key: &str, value: &str) {
    let local_storage: Storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.set_item(key, value);
}

pub async fn get(key: &str) -> Result<Option<String>, wasm_bindgen::JsValue> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage.get_item(key)
}
