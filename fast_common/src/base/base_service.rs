use async_trait::async_trait;

use rbatis::crud::{CRUD, CRUDTable};
use rbatis::{Page, PageRequest};
use rbatis::db::DBExecResult;

use rbatis::wrapper::Wrapper;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::common::orm_config::RB;
use crate::models::user::User;

#[async_trait]
pub trait BaseService: Sync + Send {
    type Model: CRUDTable + Serialize + DeserializeOwned;


    fn get_wrapper(arg: &Self::Model) -> Wrapper;

    async fn save(model: User) -> anyhow::Result<DBExecResult> {
        let x = RB.save(&model, &[]).await?;
        Ok(x)
    }

    async fn update(arg: &Self::Model, model: &Self::Model) -> anyhow::Result<u64> {
        let wrapper = Self::get_wrapper(&arg);
        let result = RB.update_by_wrapper(&model, wrapper, &[]).await?;
        Ok(result)
    }

    async fn delete(arg: &Self::Model) -> anyhow::Result<u64> {
        let wrapper = Self::get_wrapper(&arg);
        let result = RB.remove_by_wrapper::<Self::Model>(wrapper).await?;
        Ok(result)
    }


    /// 默认分页实现
    async fn page(arg: &Self::Model, page_num: Option<u64>, page_size: Option<u64>) -> anyhow::Result<Page<Self::Model>> {
        let wrapper = Self::get_wrapper(arg);
        //构建分页条件
        let page_request = PageRequest::new(page_num.unwrap_or(1), page_size.unwrap_or(10));
        //执行分页查询
        let pages: Page<Self::Model> = RB.fetch_page_by_wrapper(wrapper, &page_request).await?;
        Ok(pages)
    }

    ///默认列表实现
    async fn list(arg: &Self::Model) -> anyhow::Result<Vec<Self::Model>> {
        let wrapper = Self::get_wrapper(arg);
        let list: Vec<Self::Model> = RB.fetch_list_by_wrapper(wrapper).await?;
        Ok(list)
    }
}