use std::collections::HashMap;

use actix_web::{
    get,
    web::{self, Path, Query},
    HttpRequest, HttpResponse,
};
use tera::Tera;

use crate::{
    common::api_result::Api,
    models::student::{get_student, get_students},
};

#[get("/students")]
pub async fn students(request: HttpRequest, template: web::Data<Tera>) -> HttpResponse {
    let tmpl_name = "students.html";
    let mut context = tera::Context::new();

    let students = get_students().await.unwrap();
    context.insert("students", &students);
    let body = template.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}
#[get("/edit-student")]
pub async fn edit_student(
    request: HttpRequest,
    student_id: Query<HashMap<String, i64>>,
    template: web::Data<Tera>,
) -> HttpResponse {
    let tmpl_name = "edit-student.html";
    let mut context = tera::Context::new();

    context.insert(
        "student", &get_student(*student_id.get("studentId").unwrap()).await.unwrap());

    let body = template.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

 


#[get("/add-student")]
pub async fn add_student(request: HttpRequest, template: web::Data<Tera>) -> HttpResponse {
    let tmpl_name = "add-student.html";
    let mut context = tera::Context::new();
    let body = template.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}
#[get("/student-details")]
pub async fn student_details(request: HttpRequest, template: web::Data<Tera>) -> HttpResponse {
    let tmpl_name = "student-details.html";
    let mut context = tera::Context::new();
    let body = template.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}
