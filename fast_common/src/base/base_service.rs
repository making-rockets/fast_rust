use async_trait::async_trait;

use rbatis::crud::{CRUD, CRUDTable};
use rbatis::{Page, PageRequest};
use rbatis::db::DBExecResult;

use rbatis::wrapper::Wrapper;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::common::orm_config::RB;

#[async_trait]
pub trait BaseService<Model, Params>: Sync + Send where Model: CRUDTable + Serialize + DeserializeOwned, Params: Serialize + Sync + Send {
    fn get_wrapper(arg: &Params) -> Wrapper;

    async fn save(&self, model: &Model) -> anyhow::Result<DBExecResult> {
        let x = RB.save(&model, &[]).await?;
        Ok(x)
    }

    async fn update(&self, arg: &Params, model: &Model) -> anyhow::Result<u64> {
        let wrapper = Self::get_wrapper(&arg);
        let result = RB.update_by_wrapper(&model, wrapper, &[]).await?;
        Ok(result)
    }

    async fn delete(&self, arg: &Params) -> anyhow::Result<u64> {
        let wrapper = Self::get_wrapper(&arg);
        let result = RB.remove_by_wrapper::<Model>(wrapper).await?;
        Ok(result)
    }


    /// 默认分页实现
    async fn page(&self, arg: &Params, page_num: Option<u64>, page_size: Option<u64>) -> anyhow::Result<Page<Model>> {
        let wrapper = Self::get_wrapper(arg);
        //构建分页条件
        let page_request = PageRequest::new(page_num.unwrap_or(1), page_size.unwrap_or(10));
        //执行分页查询
        let pages: Page<Model> = RB.fetch_page_by_wrapper(wrapper, &page_request).await?;
        Ok(pages)
    }

    ///默认列表实现
    async fn list(&self, arg: &Params) -> anyhow::Result<Vec<Model>> {
        let wrapper = Self::get_wrapper(arg);
        let list: Vec<Model> = RB.fetch_list_by_wrapper(wrapper).await?;
        Ok(list)
    }
}