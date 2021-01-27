use actix_web::Result;
use log::{debug, error, info, trace, warn, LevelFilter};
use rbatis::core::db::DBPoolOptions;
use rbatis::core::Error;
use rbatis::plugin::intercept::SqlIntercept;
use rbatis::plugin::log::LogPlugin;
use rbatis::rbatis::Rbatis;
use serde::de::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

lazy_static! {
    pub static ref RB: Rbatis = {
        let mut rbatis = Rbatis::new();
        rbatis.sql_intercepts.push(Box::new(Intercept {}));
        rbatis.set_log_plugin(RbatisLog {});

        println!("rbatis init success!");
        return rbatis;
    };
    pub static ref DB_POOL_OPTIONS: DBPoolOptions = {
        let mut opt = DBPoolOptions::new();
        opt.max_connections = 100;
        opt.connect_timeout = Duration::from_secs(1000);
        return opt;
    };
    /*pub &client: Client = {
        let client: Client = redis::Client::open("redis://127.0.0.1").unwrap();

        return client;
    };*/
}

#[derive(Debug)]
pub struct Intercept {}

impl SqlIntercept for Intercept {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<serde_json::Value>,
        is_prepared_sql: bool,
    ) -> std::result::Result<(), Error> {
        println!("SQL Interceptor execute sql= {}, args= {:?}", &sql, &args);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RbatisLog {}

impl LogPlugin for RbatisLog {
    fn get_level_filter(&self) -> &LevelFilter {
        &LevelFilter::Debug
    }

    fn error(&self, data: &str) {
        error!("sql log error = {}", data);
    }

    fn warn(&self, data: &str) {
        warn!("sql log warn = {}", data);
    }

    fn info(&self, data: &str) {
        info!("sql log info = {}", data);
    }

    fn debug(&self, data: &str) {
        debug!("sql log debug = {}", data);
    }

    fn trace(&self, data: &str) {
        trace!("sql log trace = {}", data);
    }
}
