 
use serde::{de, Serialize};
use sql_builder::SqlBuilder;
use sqlx::{  Pool  , Row, Sqlite};

pub mod menu;
pub mod role;
pub mod user;

// #[async_trait]
// pub trait PageInfo<T>
// where
//     T: Clone + Send + Sync + Serialize + de::DeserializeOwned,
// {
//     async fn get_table_name() -> String;

//     async fn get_page(t: &T) -> Page<T> {
//         unimplemented!()
//     }

//     async fn get_list(t: &T) -> Vec<T> {
//         let table_name = Self::get_table_name().await;

//         unimplemented!()
//     }

//     async fn get_by_id(id: i64) -> T {
//         unimplemented!()
//     }

//     async fn edit_by_id() -> anyhow::Result<i64> {
//         unimplemented!()
//     }

//     async fn delete_by_id(id: i64) -> anyhow::Result<i64> {
//         todo!()
//     }
// }

#[derive(Clone, Debug, Serialize)]
pub struct Page<T: Clone + Serialize> {
    item_total: i64,
    list: Vec<T>,
    current_page: i64,
    current_size: i64,
    page_total: i64,
}

impl<T> Page<T>
where
    T: Clone + Serialize,
{
    pub async fn new(current_page: i64, current_size: i64, item_total: i64) -> Page<T> {
        let page_total = (item_total + 1) / current_size + 1;
        Page {
            item_total,
            list: vec![],
            current_page,
            current_size,
            page_total,
        }
    }

    pub fn add_data(&mut self, list: Vec<T>) -> Page<T> {
        self.list = list;
        self.to_owned()
    }

    pub async fn page_info(
        sql_builder: &SqlBuilder,
        current_page: i64,
        current_size: i64,
        pool: &Pool<Sqlite>,
    ) -> anyhow::Result<Page<T>>
    where
        T: Clone + Serialize + de::DeserializeOwned,
    {
        let sql = sql_builder.sql().unwrap();
        println!("分页SQL = {sql}");
        let page_sql_index = sql.find("FROM").unwrap();

        let page_sql = &("select count(*) as count ".to_string() + &sql[page_sql_index..]);
        let map = sqlx::query(page_sql).fetch_one(pool).await?;
        let count = map.get::<i64, &'static str>("count");
        let page: Page<T> = Page::new(current_page, current_size, count).await;
        Ok(page)
    }
}

pub fn build_limit(
    sql_builder: &mut SqlBuilder,
    current_page: i64,
    current_size: i64,
) -> &mut SqlBuilder {
    if current_page == 1 {
        let sql_builder = sql_builder.limit(current_size).offset(1);
        return sql_builder;
    } else {
        let sql_builder = sql_builder
            .limit(current_size)
            .offset((current_page - 1) * current_size);
        return sql_builder;
    }
}
