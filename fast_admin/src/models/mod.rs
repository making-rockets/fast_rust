use async_trait::async_trait;
use futures_util::future::ok;
use serde::{de, Deserialize, Serialize};
use serde::de::value::UsizeDeserializer;
use sqlx::{Encode, Executor, Pool, QueryBuilder, Row, Sqlite};

pub mod menu;
pub mod role;
pub mod role_menu;
pub mod student;
pub mod user;
pub mod user_role;

#[async_trait]
pub trait PageInfo<T> where T: Clone + Send + Sync + Serialize + de::DeserializeOwned {
    async fn get_table_name() -> String;

    async fn get_page(t: &T) -> Page<T> {
        unimplemented!()
    }

    async fn get_list(t: &T) -> Vec<T> {
        let table_name = Self::get_table_name().await;

        unimplemented!()
    }

    async fn get_by_id(id: i64) -> T {
        unimplemented!()
    }

    async fn edit_by_id() -> anyhow::Result<i64> {
        unimplemented!()
    }

    async fn delete_by_id(id: i64) -> anyhow::Result<i64> {
        todo!()
    }
}


#[derive(Clone, Debug)]
pub struct Page<T: Clone + Serialize + de::DeserializeOwned> {
    item_total: i64,
    list: Vec<T>,
    current_page: i64,
    current_size: i64,
    page_total: i64,
}

impl<T> Page<T> where T: Clone + Serialize + de::DeserializeOwned {
    pub async fn new(current_page: i64, current_size: i64, item_total: i64) -> Page<T> {
        let page_total = (item_total + 1) / current_size;
        Page {
            item_total,
            list: vec![],
            current_page,
            current_size,
            page_total,
        }
    }

    pub async fn add_data(&mut self, list: Vec<T>) -> Page<T> {
        self.list = list;
        self.to_owned()
    }
}


pub async fn page_info<T>(sql: String, current_page: i64, current_size: i64, list: Vec<T>, pool: &Pool<Sqlite>) -> anyhow::Result<Page<T>> where T: Clone + Serialize + de::DeserializeOwned {
    let page_sql_index = sql.find("from").unwrap();
    let page_sql = "select count(*) as count ".to_string() + &sql[page_sql_index..];
    println!("{}", page_sql);
    let map = sqlx::query(page_sql.as_str()).fetch_one(pool).await?;
    let count = map.get::<i64, &'static str>("count");
    let mut page: Page<T> = Page::new(current_page, current_size, count).await;
    page = page.add_data(list).await;
    return Ok(page);
}

pub fn build_limit(mut query_builder:  QueryBuilder<Sqlite>, current_page: i64, current_size: i64) -> QueryBuilder<Sqlite>{
    query_builder.push(" limit ").push((current_page) * current_size).push(" offset ").push((current_page - 1));
    return query_builder;
}