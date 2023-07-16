use anyhow::Ok;
use serde::{Deserialize, Serialize};
use sql_builder::{prelude::Bind, quote, SqlBuilder};

use sqlx::{FromRow, Pool, Sqlite};

use crate::models::{build_limit, Page};

#[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, PartialEq, FromRow)]
pub struct User {
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub create_time: Option<String>,
    pub password: Option<String>,
    pub status: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct LoginUserForm {
    pub user_name: Option<String>,
    pub password: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DeleteUsers {
    pub user_ids: Vec<i64>,
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
    pub async fn get_user_by_user_id(
        user_id: i64,
        pool: &Pool<Sqlite>,
    ) -> anyhow::Result<Option<User>> {
        let sql = SqlBuilder::select_from("user")
            .field("*")
            .and_where_eq("user_id", user_id)
            .sql()?;
        let u = sqlx::query_as(&sql).fetch_one(pool).await?;
        Ok(Some(u))
    }
    // 添加用户逻辑
    pub async fn add_user(mut user: User, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        if user.create_time.is_none() {
            let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            user.create_time = Some(current_time);
        }
        println!("这是是为什么{:?}", &user);
        let sql = SqlBuilder::insert_into("user")
            .set_fields(&["user_name", "password", "create_time", "status"])
            .values(&[
                &quote(user.user_name.unwrap()),
                &quote(user.password.unwrap()),
                &quote(user.create_time.unwrap()),
                &quote(user.status.unwrap()),
            ])
            .sql()?;

        let add_user = sqlx::query(&sql).execute(pool).await?;

        Ok(add_user.last_insert_rowid())
    }
    // 编辑用户逻辑
    pub async fn edit_user(user: User, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        let mut update_builder = SqlBuilder::update_table("user");
        if user.user_name.is_some() {
            update_builder.set("user_name", user.user_name.unwrap());
        }
        if user.status.is_some() {
            update_builder.set("status", user.status.unwrap());
        }
        if user.password.is_some() {
            update_builder.set("password", user.password.unwrap());
        }
        let sql = update_builder
            .and_where("user_id = ?".bind(&user.user_id.unwrap()))
            .sql()?;

        let result = sqlx::query(&sql).execute(pool).await;

        Ok(result.unwrap().rows_affected() as i64)
    }
    // 删除用户
    pub async fn delete_user(user_id: i64, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        let sql = SqlBuilder::delete_from("user")
            .and_where_eq("user_id", user_id)
            .sql()?;
        let result = sqlx::query(&sql).execute(pool).await;
        Ok(result.unwrap().rows_affected() as i64)
    }

    // 用户列表逻辑
    pub async fn user_list(user: ReqPageUserVo, pool: &Pool<Sqlite>) -> anyhow::Result<Vec<User>> {
        let mut sql_builder = SqlBuilder::select_from("user");
        if user.user_name.is_some() {
            sql_builder.and_where_like_any("user_name", user.user_name.unwrap());
        }
        if user.status.is_some() {
            sql_builder.and_where_eq("status", user.status.unwrap());
        }
        if user.user_id.is_some() {
            sql_builder.and_where_eq("user_id", user.user_id.unwrap());
        }
        if user.start_time.is_some() && user.end_time.is_some() {
            sql_builder.and_where_between(
                "create_time",
                user.start_time.unwrap(),
                user.end_time.unwrap(),
            );
        }
        let sql = sql_builder.order_desc("create_time").sql()?;

        let list = sqlx::query_as::<Sqlite, User>(&sql).fetch_all(pool).await?;
        Ok(list)
    }

    // 用户列表分页
    pub async fn user_page(
        req_page_user: ReqPageUserVo,
        pool: &Pool<Sqlite>,
    ) -> anyhow::Result<Page<User>> {
        let mut sql_builder = SqlBuilder::select_from("user");

        if req_page_user.user_name.is_some() {
            sql_builder.and_where_like_any("user_name", req_page_user.user_name.unwrap());
        }
        if req_page_user.status.is_some() {
            sql_builder.and_where_eq("status", req_page_user.status.unwrap());
        }
        if req_page_user.start_time.is_some() && req_page_user.end_time.is_some() {
            sql_builder.and_where_between(
                "create_time",
                req_page_user.start_time.unwrap(),
                req_page_user.end_time.unwrap(),
            );
        }

        let current_page = req_page_user.current_page.unwrap_or(1);
        let current_size = req_page_user.current_size.unwrap_or(10);

        let mut page_info = Page::page_info(&sql_builder, current_page, current_size, pool).await?;

        let sql_builder = build_limit(&mut sql_builder, current_page, current_size);

        let sql = sql_builder.sql()?;

        let list = sqlx::query_as::<Sqlite, User>(&sql).fetch_all(pool).await?;

        page_info.add_data(list);
        Ok(page_info)
    }
    // 删除所有用户
    pub async fn delete_users(arg: DeleteUsers, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        let sql = SqlBuilder::delete_from("user")
            .and_where_in("user_id", &arg.user_ids)
            .sql()?;

        let query_result = sqlx::query(&sql).execute(pool).await?;
        Ok(query_result.rows_affected() as i64)
    }
}
