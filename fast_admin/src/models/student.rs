use std::fmt::format;
use std::process::id;
use actix_web::web;
use actix_web::web::Data;
use chrono::NaiveDateTime;
use mysql_async::{Conn, params};
use mysql_async::prelude::{BatchQuery, Query, WithParams};

use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<String>,
    pub name: String,
    pub class: String,
    pub mobile: String,
    pub address: String,
    //pub create_time:NaiveDateTime
}

pub async fn insert_student(student: Student,  conn: &mut Data<Conn>) {
  let result =    r"INSERT INTO student (id, name, class,mobile,address)
      VALUES (:id, :name, :class,:mobile,:address)"
        .with(params! {
            "id" => student.id.unwrap(),
            "name" => student.name,
            "class" => student.class,
            "mobile" => student.mobile,
            "address" => student.address,
        }).run( conn.into_inner(). ).await.unwrap().;
}


pub async fn get_students() -> anyhow::Result<Vec<Student>> {
    let mut studetns = Vec::new();
    for i in 0..10 {
        let student = Student {
            id: Some(i.to_string()),
            name: format!("{}{}", i, "名称"),
            class: format!("{}{}", i, "班级"),
            mobile: format!("{}{}", i, "手机号"),
            address: format!("{}{}", i, "地址"),
            // create_time: NaiveDateTime::default(),
        };

        studetns.push(student);
    }

    Ok(studetns)
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
