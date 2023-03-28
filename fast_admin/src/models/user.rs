use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use redis::{AsyncCommands, ToRedisArgs};

use serde::{Deserialize, Deserializer, Serialize};

use sqlx::{database, Encode, Execute, Executor, FromRow, Pool, query, query_as, QueryBuilder, Sqlite, SqliteConnection, Type};
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;

use sqlx::sqlite::SqliteTypeInfo;
use tracing_subscriber::fmt::format;

use crate::models::{build_limit, Page, PageInfo};


#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub create_time: Option<String>,
    pub password: Option<String>,
    pub status: Option<i64>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReqPageUserVo {
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub create_time: Option<String>,
    pub status: Option<i64>,

    pub current_page: Option<i64>,
    pub current_size: Option<i64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}


impl User {
    // 通过user_id 获取单个对象值
    pub async fn get_user_by_user_id(user_id: i64, pool: &Pool<Sqlite>) -> anyhow::Result<Option<User>> {
        let u = sqlx::query_as(&format!("select * from user where user_id={}", user_id)).fetch_one(pool).await?;
        Ok(Some(u))
    }
    // 添加用户逻辑
    pub async fn add_user(mut user: User, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        if user.create_time.is_none() {
            let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            user.create_time = Some(current_time);
        }
        let add_user = sqlx::query("INSERT INTO user(user_name, password, create_time, status)values($1,$2,$3,$4)").bind(user.user_name).bind(user.password).bind(user.create_time).bind(user.status).execute(pool).await?;
        Ok(add_user.last_insert_rowid())
    }
    // 编辑用户逻辑
    pub async fn edit_user(mut user: User, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        let mut sql_builder = QueryBuilder::<Sqlite>::new("update user set ");
        if user.user_name.is_some() {
            sql_builder.push(&format!(" user_name = '{}',", user.user_name.unwrap()));
        }
        if user.status.is_some() {
            sql_builder.push(&format!("status = {},", user.status.unwrap()));
        }
        if user.password.is_some() {
            sql_builder.push(&format!("password = '{}',", user.password.unwrap()));
        }

        let mut trim_sql = sql_builder.sql().to_string();
        trim_sql.pop();

        let result = sqlx::query(&format!("{} where user_id ={}", trim_sql, user.user_id.unwrap())).execute(pool).await;
        Ok(result.unwrap().rows_affected() as i64)
    }
    // 删除用户
    pub async fn delete_user(user_id: i64, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        let result = sqlx::query(&format!("delete from user where user_id = {}", user_id)).execute(pool).await;
        Ok(result.unwrap().rows_affected() as i64)
    }


    // 用户列表逻辑
    pub async fn user_list(user: ReqPageUserVo, pool: &Pool<Sqlite>) -> anyhow::Result<Vec<User>> {
        let mut query_builder: QueryBuilder<Sqlite> = sqlx::QueryBuilder::new("select * from user where 1=1 ");
        if user.user_name.is_some() {
            query_builder.push(format!(" and user_name like '%{}%'", user.user_name.unwrap()));
        }
        if user.status.is_some() {
            query_builder.push(format!(" and status = {}", user.status.unwrap()));
        }
        if user.start_time.is_some() {
            query_builder.push(format!(" and create_time >  '{}'", user.start_time.unwrap()));
        }
        if user.end_time.is_some() {
            query_builder.push(format!(" and create_Time < '{}' ", user.end_time.unwrap()));
        }
        query_builder.push(" order by create_time desc ");

        let list = sqlx::query_as::<Sqlite, User>(query_builder.sql()).fetch_all(pool).await?;
        Ok(list)
    }

    // 用户列表分页
    pub async fn user_page(req_page_user: ReqPageUserVo, pool: &Pool<Sqlite>) -> anyhow::Result<Page<User>> {
        let mut query_builder: QueryBuilder<Sqlite> = sqlx::QueryBuilder::new("select  * from user where 1=1 ");
        if req_page_user.user_name.is_some() {
            query_builder.push(format!(" and user_name like '%{}%'", req_page_user.user_name.unwrap()));
        }
        if req_page_user.status.is_some() {
            query_builder.push(" and status = ").push(req_page_user.status.unwrap());
        }
        if req_page_user.start_time.is_some() {
            query_builder.push(format!(" and create_time >  '{}'", req_page_user.start_time.unwrap()));
        }
        if req_page_user.end_time.is_some() {
            query_builder.push(format!(" and create_Time < '{}' ", req_page_user.end_time.unwrap()));
        }
        query_builder.push(" order by create_time desc ");

        let current_page = req_page_user.current_page.unwrap_or(1);
        let current_size = req_page_user.current_size.unwrap_or(10);
        query_builder = build_limit(query_builder, current_page, current_size);

        let query = query_builder.build_query_as::<User>();
        //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
        let sql = query.sql();

        let list = sqlx::query_as::<Sqlite, User>(sql).fetch_all(pool).await?;
        let mut page_info = Page::page_info(sql.to_string(), current_page, current_size, list, pool).await?;
        Ok(page_info)
    }
}
