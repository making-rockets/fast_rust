use std::borrow::BorrowMut;
use std::collections::HashMap;

use actix_web::{get, post, web::{self, Path, Query}, HttpRequest, HttpResponse, Responder};
use tera::{Template, Tera};

use crate::{common::api_result::Api, GLOBAL_TERA, models::student::{get_student, get_students}};
use crate::models::student::{insert_student, Student};

#[get("/students")]
pub async fn students(request: HttpRequest) -> HttpResponse {
    let tmpl_name = "students.html";
    let mut context = tera::Context::new();

    let students = get_students().await.unwrap();
    context.insert("students", &students);
    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

#[get("/edit-student")]
pub async fn edit_student(
    request: HttpRequest,
    student_id: Query<HashMap<String, i64>>,
) -> HttpResponse {
    let tmpl_name = "edit-student.html";
    let mut context = tera::Context::new();

    context.insert(
        "student", &get_student(*student_id.get("studentId").unwrap()).await.unwrap());

    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}
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

#[get("/add-student")]
pub async fn add_student(request: HttpRequest) -> HttpResponse {
    let tmpl_name = "add-student.html";
    let mut context = tera::Context::new();
    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

#[post("/add-student-submit")]
pub async fn add_student_submit(student: web::Form<Student>) -> HttpResponse {


    //添加数据
    insert_student(student.into_inner()).await;


    let tmpl_name = "students.html";
    let mut context = tera::Context::new();
    //添加完数据后返回数据库中的所有数据
    context.insert("students", &get_students().await.unwrap());
    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}


#[get("/student-details")]
pub async fn student_details(request: HttpRequest) -> HttpResponse {
    let tmpl_name = "student-details.html";
    let mut context = tera::Context::new();
    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}
