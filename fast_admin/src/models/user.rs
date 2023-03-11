use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite, SqliteConnection};


#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub create_time: Option<String>,
    pub password: Option<String>,
    pub status: Option<i64>,
}


impl User {
    pub async fn get_user(user_id: i64, pool: &Pool<Sqlite>) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<Sqlite, User>("select * from `user` where user_id =$1").bind(user_id).fetch_one(pool).await;
        println!("{:?}", user);
        if let Ok(u) = user {
            return Ok(Some(u));
        }
        Ok(None)
    }
    pub async fn add_user(user: User) -> anyhow::Result<i64> {

        // let add_user = sqlx::query("INSERT INTO user(user_name, password, create_time, status)values($1,$2,$3,$4)").bind(user.user_name).bind(user.password).bind(user.create_time).bind(user.status).execute(a).await?;
        // Ok(add_user.last_insert_rowid())
        todo!()
    }

    pub async fn user_page(user: User) -> anyhow::Result<Vec<User>> {

        // let result = sqlx::query_as::<Sqlite, User>("select * from user ").fetch_all(a).await?;
        // Ok(result)
        todo!()
    }
}