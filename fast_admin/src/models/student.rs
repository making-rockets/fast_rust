use std::fmt::format;
use std::process::id;
use actix_web::web;
use actix_web::web::Data;
use chrono::{DateTime, Local, NaiveDateTime};
use futures_util::pin_mut;

use redis::Client;

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::{PG_POOL};
use crate::controller::student_controller::student_details;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<String>,
    pub name: Option<String>,
    pub class: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub create_time: Option<DateTime<Local>>,
}

pub async fn insert_student(student: Student) {
    let client = PG_POOL.get().await.unwrap();

    let statment = client.prepare("insert into student(id,name,class,mobile,address,create_time) values ($1,$2,$3,$4,%5,$6)").await.unwrap();
    client.execute(&statment, &[&100, &student.name.unwrap(), &student.class.unwrap(), &student.mobile.unwrap(), &student.address.unwrap(),
        &chrono::Local::now()
    ]);
}


pub async fn get_students() -> anyhow::Result<Vec<Student>> {
    Ok(vec![])
}

pub async fn get_student(student_id: i64) -> anyhow::Result<Student> {
    Ok(todo!())
}
