use actix_web::web;
use actix_web::web::Data;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use futures_util::future::ok;
use futures_util::pin_mut;
use std::fmt::format;
use std::process::id;

use redis::Client;

use crate::controller::student_controller::student_details;

use serde::{Deserialize, Serialize};
use serde_json::to_string;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: Option<String>,
    pub name: Option<String>,
    pub class: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,

    pub create_time: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}

mod my_date_format {
    use std::process::id;
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};


    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer, {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        let result = DateTime::parse_from_str(s.as_str(), "%Y-%m-%d %H:%M:%S").unwrap().naive_local();
        Ok(result)
    }
}

pub async fn insert_student(student: Student) {
    let create_time = student.create_time.unwrap().replace('T', " ");
    let create_time =
        chrono::NaiveDateTime::parse_from_str(create_time.as_str(), "%Y-%m-%d %H:%M").unwrap();

    //let statement = "".prepare("INSERT INTO student(id,name,class,mobile,address,create_time)VALUES($1,$2,$3,$4,$5,$6)").await;
}

pub async fn get_students(page_num: &i64, page_size: &i64) -> anyhow::Result<Vec<Student>> {
    todo!()
}

pub async fn get_student(student_id: i64) -> anyhow::Result<Student> {
    todo!()
}
