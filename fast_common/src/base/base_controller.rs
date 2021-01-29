
use crate::base::base_model::BaseModel;
use crate::base::base_service::BaseService;
use actix_web::HttpResponse;
use actix_web::web::Form;
use std::collections::HashMap;
pub trait BaseController {

    type Model:BaseModel;
    type Service: BaseService<Self::Model>;

    fn save_for_create(post: Form<HashMap<String, String>>) -> HttpResponse {
        let post_fields = post.into_inner();
        if let Err(message) = Self::M::validate(&post_fields) {  //如果检验出错
            return response::error(message);
        }
        let table_name = Self::M::get_table_name();
        let table_fields = caches::TABLE_FIELDS.lock().unwrap();
        let mut checked_fields = Db::check_fields(table_name, &table_fields, post_fields, false); //經過檢驗之後的數據
        Self::M::save_before(&mut checked_fields); //对于保存数据前的检测
        let mut data = DataSet::create();
        for (k, v) in &checked_fields {
            data.set(k, &v.trim());
        }
        let mut conn = db::get_conn();
        let id = Self::M::create(&mut conn, &data);
        if id > 0 {
            return response::ok();
        }
        response::error("增加記錄失敗")
    }

    fn update();
    fn delete();
    fn list();
    fn list_page();
}