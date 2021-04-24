use fast_common::models::user::User;
use fast_common::models::role::{RoleVo, Role};
use fast_common::common::orm_config::RB;
use fast_common::models::user_role::UserRole;
use rbatis::Result;


pub struct RoleService {}

//TODO
impl RoleService {
    pub async fn find_role_by_user(user_id: u64) -> Result<Role> {
        let sql = format!("{}{}", "select a.id,a.role_name,a.index_no,a.remark,a.state,a.create_time from role a  left join user_role b on a.id = b.role_id and a.user_id =", user_id.to_string());
        let x = RB.fetch::<Role>("", sql.as_str()).await;
        return x;
    }
}