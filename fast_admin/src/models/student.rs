use std::fmt::format;
use std::process::id;
use actix_web::web;
use actix_web::web::Data;
use chrono::NaiveDateTime;
use redis::Client;

use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::{PG_POOL};


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
    let client = PG_POOL.get().await.unwrap();
    let statement = client.prepare("select * from student").await.unwrap();
    let result = client.query(&statement, &[]).await;
    for x in result.unwrap().iter() {
        println!("{:?}", x.get::<usize,i64 >(0 ));
    }
}


pub async fn get_students() -> anyhow::Result<Vec<Student>> {
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
