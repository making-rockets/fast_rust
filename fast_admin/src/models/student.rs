use std::fmt::format;
use std::process::id;
use actix_web::web;
use actix_web::web::Data;
use chrono::NaiveDateTime;
use mysql_async::{Conn, params, QueryResult, Result};
use mysql_async::prelude::{BatchQuery, FromRow, Protocol, Query, Queryable, WithParams};

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::{GLOBAL_CONN_LAZY_STATIC, GLOBAL_CONN_ONCE_CELL};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<String>,
    pub name: String,
    pub class: String,
    pub mobile: String,
    pub address: String,
    //pub create_time:NaiveDateTime
}

pub async fn insert_student(student: Student) {
    let mut conn = GLOBAL_CONN_ONCE_CELL.get_conn().await.unwrap();
    let result = r"INSERT INTO student (id, name, class,mobile,address)
      VALUES (:id, :name, :class,:mobile,:address)"
        .with(params! {
            "id" => 1,
            "name" => student.name,
            "class" => student.class,
            "mobile" => student.mobile,
            "address" => student.address,
        }).run(&mut conn).await;
    println!("{:?}", result);
}

pub async fn get_all_results<TupleType, P>(mut result: QueryResult<'_, '_, P>) -> Result<Vec<TupleType>>
    where TupleType: FromRow + Send + 'static, P: Protocol + Send + 'static, {
    Ok(result.collect().await?)
}


pub async fn get_students() -> anyhow::Result<Vec<Student>> {
    let mut conn = GLOBAL_CONN_ONCE_CELL.get_conn().await.unwrap();

    let result = conn.query("select * from student").await?;

   let result =  get_all_results(result).await?;

    let students = "SELECT * FROM student"
        .with(())
        .map(&mut conn, |(id, &name, class, mobile, address, )| Student {
            id: Some(id),
            name: serde_json::to_string(name).unwrap(),
            class: serde_json::to_string(class).unwrap(),
            mobile: serde_json::to_string(mobile).unwrap(),
            address: serde_json::to_string(address).unwrap(),
        },
        ).await?;
    Ok(vec![])
}

pub async fn get_student(student_id: i64) -> anyhow::Result<Student> {
    Ok(Student {
        id: Some(student_id.to_string()),
        name: "两拨1".to_owned(),
        class: "两拨2".to_owned(),
        mobile: "两拨5".to_owned(),
        address: "两拨6".to_owned(),
        // create_time: NaiveDateTime::default(),
    })
}
