use fast_common::models::user::User;
use fast_common::models::role::{RoleVo, Role};
use fast_common::common::orm_config::RB;
use fast_common::models::user_role::UserRole;


pub struct RoleService {}

impl RoleService {
    pub async fn find_role_by_user(user: User) -> UserRole {
        let x = RB.fetch::<RoleVo>("", "").await;
        let wrapper = RB.new_wrapper_table::<Role>();
        wrapper.
    }
}