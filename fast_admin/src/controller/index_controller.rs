use actix_web::web::{Data, Json, Query};
use actix_web::HttpResponse;
use actix_web::{get, post, HttpRequest};

use sqlx::{Pool, Sqlite};

use crate::common::api_result::Api;

use crate::models::user::{LoginUserForm, User};
use crate::utils::redis_util::REDIS_UTIL;
use crate::GLOBAL_TERA;

#[get("/send_reg_code")]
pub async fn push_reg_code(user_name: Query<()>) -> HttpResponse {
    todo!()
}

#[post("/do_login")]
pub async fn login(user_login_form: Json<LoginUserForm>) -> HttpResponse {
    println!("user_login_form = {:?}", user_login_form);

    let mut context = tera::Context::new();
    let body = GLOBAL_TERA.render("login.html", &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

#[post("/login_submit")]
pub async fn login_submit(
    login_form: Json<LoginUserForm>,
    pool: Data<Pool<Sqlite>>,
    _request: HttpRequest,
) -> HttpResponse {
    let other_login = login_form.0;
    if other_login.user_name.is_none() || other_login.password.is_none() {
        return Api::<String>::error("未知的用户名和密码".to_string())
            .await
            .to_response_of_json()
            .await;
    }

    //根据用户名查询用户数据
    let user_result = sqlx::query_as::<Sqlite, User>("select * from user where user_name = $1")
        .bind(other_login.user_name.unwrap())
        .fetch_one(pool.as_ref())
        .await;

    if user_result.is_err() {
        return Api::<String>::error("数据服务错误".to_string())
            .await
            .to_response_of_json()
            .await;
    }
    let user = user_result.unwrap();
    if &0 == &user.status.unwrap() {
        return Api::<String>::error("用户被禁用".to_string())
            .await
            .to_response_of_json()
            .await;
    }
    if !user
        .clone()
        .password
        .unwrap()
        .eq(&other_login.password.unwrap())
    {
        return Api::<String>::error("密码错误".to_string())
            .await
            .to_response_of_json()
            .await;
    }

    //生成token
    let token = "asdfsadfasdfasdfasdf";

    let mut context = tera::Context::new();
    context.insert("current_user", &user);
    context.insert("token", &token);
    let body = GLOBAL_TERA.render("index.html", &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

#[get("")]
pub async fn index(request: HttpRequest, data: Data<Pool<Sqlite>>) -> HttpResponse {
    let tmpl_name = "index.html";
    let mut context = tera::Context::new();
    let body = GLOBAL_TERA.render(tmpl_name, &context).unwrap();
    Api::<String>::success()
        .await
        .to_response_of_html(body)
        .await
}

#[get("/logout")]
pub async fn logout(request: HttpRequest) -> HttpResponse {
    let header = request.headers().get("authorization");
    match header {
        None => {
            Api::<()>::error(String::from("未登录"))
                .await
                .to_response_of_json()
                .await
        }
        Some(access_token) => {
            REDIS_UTIL.delete(access_token.to_str().unwrap()).await;
            Api::<()>::success().await.to_response_of_json().await
        }
    }
}
