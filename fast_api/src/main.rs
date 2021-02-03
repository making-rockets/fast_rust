use futures::future::join_all;
use reqwest::{Response, Error};
use std::future::Future;

async fn fetch_path(path: String) -> Result<(), reqwest::Error> {
    match reqwest::get(&path).await {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) =>{
                    println!("{:?}", text);
                    println!("{:?}",text.len())
                }
                Err(_) =>{
                    println!("error while reading {}",path)
                }
            }
        }
        Err(_) => {
            println!("error while scraping from {}",path)
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let paths = vec!["http://xxx.xxx.com/xxx".to_string(), "http://xxx.xxx.com/xxx".to_string()];

    join_all(paths.into_iter().map(|path| {
        fetch_path(path)
    })).await;
    Ok(())
}

