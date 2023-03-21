use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Duration;
use futures_util::TryFutureExt;
use reqwest::header::HeaderMap;


#[tokio::main]
pub async fn main() {
    let clientBuilder = reqwest::ClientBuilder::new();
    let mut map = HeaderMap::new();
    map.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36 MicroMessenger/7.0.20.1781(0x6700143B) NetType/WIFI MiniProgramEnv/Windows WindowsWechat/WMPF XWEB/6609".parse().unwrap());
    map.insert("Device-Info-QS", "{\"clientType\": 5}".parse().unwrap());
    map.insert("xweb_xhr", "1".parse().unwrap());

    let mut client = clientBuilder.default_headers(map).build().unwrap();

    let url = "https://api.qingshuxuetang.com/v23_1/mall/content/search?organizationId=52623&withFilter=true&type=2&trainingTypeId=4004";
    let result = client.get(url).send().await.unwrap().json::<serde_json::Value>().await.unwrap();

    let array = result.as_object().unwrap().get("data").unwrap().as_object().unwrap().get("filter").unwrap().as_object().unwrap().get("conditions").unwrap().as_array().unwrap();


    for (index, obj) in array.iter().enumerate() {
        if obj.as_object().unwrap().get("key").unwrap().eq("5000") {
            let node = obj.as_object().unwrap().get("subNodes").unwrap().as_array().unwrap();
            for (sub_index, sub_node) in node.iter().enumerate() {
                let key = sub_node.as_object().unwrap().get("key").unwrap().as_str().unwrap();
                let name = sub_node.as_object().unwrap().get("name").unwrap().as_str().unwrap();

                let sub_url = format!("https://api.qingshuxuetang.com/v23_1/mall/content/search?organizationId=52623&withFilter=false&type=2&trainingTypeId={}", key);
                let value = client.get(sub_url).send().await.unwrap().json::<serde_json::Value>().await.unwrap();
                let sub_result = value.as_object().unwrap().get("data").unwrap().get("content").unwrap().as_object().unwrap().get("items").unwrap().as_array().unwrap();

                for (sub_item_index, sub_item) in sub_result.iter().enumerate() {
                    let file_name = sub_item.as_object().unwrap().get("name").unwrap().as_str().unwrap();
                    let file_url = sub_item.as_object().unwrap().get("url").unwrap().as_str().unwrap();
                    std::fs::create_dir(format!("./{}", name));
                    let mut file = OpenOptions::new().create(true).append(true).open(format!("./{}/{}", name, file_name)).unwrap();

                    let bytes = client.get(file_url).send().await.unwrap().bytes().await.unwrap();
                    file.write_all(&bytes);
                }
            }
        }
    }
}

