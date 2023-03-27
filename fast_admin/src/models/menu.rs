use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use sqlx::sqlite::SqliteArguments;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Menu {
    pub menu_id: Option<i64>,
    pub menu_name: Option<String>,
    pub parent_id: Option<i64>,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub remark: Option<String>,
    pub status: Option<i64>,
    pub create_time: Option<String>,
    pub index_no: Option<i64>,
    pub user_id: Option<i64>,
    pub clazz: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct MenuVo {
    pub menu_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub user_id: Option<i64>,
    pub menu_name: Option<String>,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub remark: Option<String>,
    pub status: Option<i64>,
    pub clazz: Option<String>,
    pub sub_menu: Option<Vec<MenuVo>>,

}

impl From<Menu> for MenuVo {
    fn from(value: Menu) -> Self {
        Self {
            menu_id: value.menu_id,
            parent_id: value.parent_id,
            user_id: value.parent_id,
            menu_name: value.menu_name,
            path: value.path,
            icon: value.icon,
            remark: value.remark,
            status: value.status,
            clazz: value.clazz,
            sub_menu: None,
        }
    }
}


impl Menu {
    // pub async fn get_menu_by_user_id(user_id: i64, pool: &Pool<Sqlite>) -> anyhow::Result<Vec<MenuVo>> {
    //     let all_menus = sqlx::query_as::<Sqlite, Menu>("select * from menu where  user_id =$1")
    //         .bind(user_id).fetch_all(pool).await?;
    //
    //     let mut big_menus = Vec::<MenuVo>::new();
    //     big_menus.push(MenuVo::from(x.clone()));
    //     let mut sub_menus = Self::find_children(all_menus, x.parent_id.unwrap());
    //     Ok(big_menus)
    // }
    //
    // //查找list 下的 所有
    // fn find_children(mut menu: MenuVo, parent_id: i64) -> MenuVo {
    //     if menu.parent_id.unwrap() == parent_id {
    //         let nid = menu.parent_id.unwrap();
    //         let mut mt = Self::find_children(menu.clone(), nid);
    //         if !mt.is_empty() {
    //             menu.sub_menu = Option::from(mt);
    //         }
    //     }
    //     return menu;
    //
    //     // //let mut found = vec![];
    //     // for it in list.clone() {
    //     //     if it.parent_id.unwrap() == parent_id {
    //     //         // found.push(it.clone());
    //     //         let nid = it.menu_id.unwrap_or_default();
    //     //         let mut mt = Self::find_children(list, nid);
    //     //         if !mt.is_empty() {
    //     //             //found.append(&mut mt);
    //     //             it.
    //     //         }
    //     //     }
    //     // }
    //     // found
    // }
    //
    // fn transform_sub_menu() {}
}