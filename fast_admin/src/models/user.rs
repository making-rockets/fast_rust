use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use redis::{AsyncCommands, ToRedisArgs};

use serde::{Deserialize, Serialize};
use sqlx::{database, Encode, Execute, Executor, FromRow, Pool, query_as, QueryBuilder, Sqlite, SqliteConnection, Type};
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;

use sqlx::sqlite::SqliteTypeInfo;
use tracing_subscriber::fmt::format;

use crate::models::{Page, page_info, PageInfo};


#[derive(Debug, Serialize, Deserialize, Clone, FromRow, Encode, Type)]
pub struct User {
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub create_time: Option<String>,
    pub password: Option<String>,
    pub status: Option<i64>,
}


impl User {
    pub async fn new() -> Self {
        User {
            user_id: None,
            user_name: Some("a".to_string()),
            create_time: None,
            password: None,
            status: Some(1),
        }
    }

    pub async fn get_user_by_user_id(user_id: i64) -> anyhow::Result<Option<User>> { todo!() }


    pub async fn add_user(user: User) -> anyhow::Result<i64> {
        //   let add_user = sqlx::query("INSERT INTO user(user_name, password, create_time, status)values($1,$2,$3,$4)").bind(user.user_name).bind(user.password).bind(user.create_time).bind(user.status).execute(sql_pool).await?;
        todo!()
    }

    pub async fn user_page(user: User, current_page: i64, current_size: i64, pool: &Pool<Sqlite>) -> anyhow::Result<Page<User>> {
        let mut query_builder: QueryBuilder<Sqlite> = sqlx::QueryBuilder::new("select  * from user where 1=1 ");
        if user.user_name.is_some() {
            query_builder.push(format!(" and user_name like '{}'", user.user_name.unwrap()));
        }
        if user.status.is_some() {
            query_builder.push(" and status = ").push(user.status.unwrap());
        }

        query_builder.push(" limit ").push((current_page )*current_size ).push(" offset ").push((current_page -1 ));

        let query = query_builder.build_query_as::<User>();
        let sql = query.sql();

        let list = sqlx::query_as::<Sqlite, User>(sql).fetch_all(pool).await?;
        println!("{:?}", list);
        let mut page_info = page_info::<User>(sql.to_string(), current_page, current_size, list, pool).await?;

        Ok(page_info)
    }
}
