use std::borrow::BorrowMut;
use std::collections::HashMap;

use actix_web::web::{Form, Payload};
use actix_web::{
    get, post,
    web::{self, Path, Query},
    HttpRequest, HttpResponse, Responder,
};
use tera::{Template, Tera};

use crate::models::student::{insert_student, Student};
use crate::{
    common::api_result::Api,
    models::student::{get_student, get_students},
    GLOBAL_TERA,
};

#[get("/students")]
pub async fn students(page: Query<HashMap<String, i64>>) -> HttpResponse {
    let page_num = page.get("page_num").unwrap_or(&1);
    let page_size = page.get("page_size").unwrap_or(&10);
    let students = get_students(page_num, page_size).await.unwrap();
    let tmpl_name = "students.html";
    let mut context = tera::Context::new();
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
        "student",
        &get_student(*student_id.get("studentId").unwrap())
            .await
            .unwrap(),
    );

    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

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
    let page_num = student.page_num.unwrap_or(1);
    let page_size = student.page_size.unwrap_or(10);
    insert_student(student.into_inner()).await;

    let tmpl_name = "students.html";
    let mut context = tera::Context::new();
    //添加完数据后返回数据库中的所有数据
    context.insert(
        "students",
        &get_students(&page_num, &page_size).await.unwrap(),
    );
    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

//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
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
