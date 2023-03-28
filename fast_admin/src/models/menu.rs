use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use sqlx::sqlite::SqliteArguments;
use tera::ast::Node;


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
    pub children: Option<Vec<MenuVo>>
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
            children: None,
        }
    }
}

impl Menu {
    pub async fn add_menu(menu: Menu, pool: &Pool<Sqlite>) -> anyhow::Result<i64> {
        let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let result = sqlx::query("insert into menu(menu_name,parent_id,path,icon,remark,status,create_time,index_no,user_id,clazz)values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)")
            .bind(menu.menu_name.unwrap())
            .bind(menu.parent_id.unwrap())
            .bind(menu.path.unwrap())
            .bind(menu.icon.unwrap())
            .bind(menu.remark.unwrap())
            .bind(menu.status.unwrap())
            .bind(current_time)
            .bind(menu.index_no.unwrap())
            .bind(menu.user_id.unwrap())
            .bind(menu.clazz.unwrap()).execute(pool).await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_menu_by_user_id(user_id: i64, pool: &Pool<Sqlite>) -> anyhow::Result<Vec<MenuVo>> {
        let menu_items = sqlx::query_as::<Sqlite, Menu>("select * from menu where user_id =$1")
            .bind(user_id).fetch_all(pool).await?;

        let mut menu_tree = HashMap::<i64, Vec<MenuVo>>::new();
        for menu_item in menu_items {
            let children = menu_tree.entry(menu_item.parent_id.unwrap()).or_insert(vec![]);
            let menu_item = MenuVo {
                children: Some(vec![]),
                menu_id: menu_item.menu_id,
                menu_name: menu_item.menu_name,
                parent_id: menu_item.parent_id,
                path: menu_item.path,
                icon: menu_item.icon,
                remark: menu_item.remark,
                status: menu_item.status,
                user_id: menu_item.user_id,
                clazz: menu_item.clazz,
            };
            children.push(menu_item);
        }

        let mut result = Vec::new();
        if let Some(mut root_menu_items) = menu_tree.remove(&0) {
            for mut root_menu_item in root_menu_items {
                let children = Self::fetch_children_recursive(&mut menu_tree, &root_menu_item);
                root_menu_item.children = Some(children);
                result.push(root_menu_item);
            }
        }
        Ok(result)
    }
    fn fetch_children_recursive(menu_tree: &mut HashMap<i64, Vec<MenuVo>>, parent_menu_item: &MenuVo) -> Vec<MenuVo> {
        if let Some(children) = menu_tree.remove(&parent_menu_item.menu_id.unwrap()) {
            let mut result = Vec::new();
            for mut child in children {
                let grand_children = Self::fetch_children_recursive(menu_tree, &child);
                child.children = Some(grand_children);
                result.push(child);
            }
            result
        } else {
            vec![]
        }
    }
}



