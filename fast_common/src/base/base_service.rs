use async_trait::async_trait;
use rbatis::crud::{CRUD, CRUDTable};
use rbatis::{Page, PageRequest};

use rbatis::wrapper::Wrapper;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::common::orm_config::RB;

#[async_trait]
pub trait BaseService<Model, Params>: Sync + Send where Model: CRUDTable + Serialize + DeserializeOwned, Params: Serialize {
    fn get_wrapper(arg: &Params) -> Wrapper;

    async fn save(&self, arg: &Params) -> anyhow::Result<()> {
        Ok(())
    }

    async fn update(&self, arg: &Params) -> anyhow::Result<()> {
        Ok(())
    }

    async fn delete(&self, arg: &Params) -> anyhow::Result<()> {
        Ok(())
    }


    /// 默认分页实现
    async fn page(&self, arg: &Params, pageNum: Option<u64>, pageSize: Option<u64>) -> anyhow::Result<Page<Model>> {
        let wrapper = Self::get_wrapper(arg);
        //构建分页条件
        let page_request = PageRequest::new(pageNum.unwrap_or(1), pageSize.unwrap_or(10));
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