use fast_common::base::base_service::BaseService;
use crate::service::menu_service::MenuService;
use crate::service::role_service::RoleService;
use crate::service::user_service::UserService;

pub mod menu_service;
pub mod user_service;
pub mod role_service;


pub struct Service {
    pub menu_service: MenuService,
    pub user_service: UserService,
    pub role_service: RoleService,
}

