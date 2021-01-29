use log::{debug, error, info, trace, warn, LevelFilter};
use rbatis::core::db::DBPoolOptions;
use rbatis::core::Error;
use rbatis::plugin::intercept::SqlIntercept;
use rbatis::plugin::log::LogPlugin;
use rbatis::rbatis::Rbatis;

use std::time::Duration;

lazy_static! {
    pub static ref RB: Rbatis = InitDb::new();
}

pub struct InitDb {}

impl InitDb {
    pub fn new() -> Rbatis {
        let mut rbatis: Rbatis = Rbatis::new();
        rbatis.add_sql_intercept(Intercept {});
        rbatis.set_log_plugin(RbatisLog {});
        println!("rbatis init success");
        return rbatis;
    }

    pub fn db_option() -> DBPoolOptions {
        let mut opt = DBPoolOptions::new();
        opt.min_connections = 1;
        opt.max_connections = 100;
        opt.connect_timeout = Duration::from_secs(100);
        opt.max_lifetime = Some(Duration::from_secs(1800));
        opt.test_before_acquire = true;
        return opt;
    }
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
        &LevelFilter::Trace
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
