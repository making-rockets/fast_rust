use crate::controller::index_controller as index;
use crate::controller::menu_controller as menu;
use crate::controller::student_controller as student;
use crate::controller::user_controller as user;

use actix_web::dev::HttpServiceFactory;

use actix_web::web::{scope, service};
use actix_web::{guard, web};

pub(crate) fn user_router() -> impl HttpServiceFactory {
    web::scope("/admin/user")
        .service(user::add_user) //创建新用户
        .service(user::edit_user) //编辑用户
        .service(user::delete_user) //删除用户
        .service(user::delete_users) //删除多个用户
        .service(user::list_user) //用户列表
        .service(user::page_user) //用户分页
}

pub(crate) fn menu_router() -> impl HttpServiceFactory {
    web::scope("/admin/menu")
        .service(menu::add_menu) //创建新菜单
        .service(menu::edit_menu) //编辑菜单
        .service(menu::delete_menu) //编辑菜单
        .service(menu::list_menu) //列表菜单
}

pub(crate) fn student_router() -> impl HttpServiceFactory {
    web::scope("/admin/student")
        .service(student::students) //学生列表
        .service(student::add_student) //添加学生跳转页面
        .service(student::add_student_submit) //添加学生
        .service(student::edit_student) //编辑学生
        .service(student::student_details) //学生详情
}

pub(crate) fn index_router() -> impl HttpServiceFactory {
    web::scope("/admin/index")
        .service(index::push_reg_code) //登录发送验证码
        .service(index::login) //登录
        .service(index::login_submit)
        .service(index::index) //首页
        .service(index::logout) //登出
}
