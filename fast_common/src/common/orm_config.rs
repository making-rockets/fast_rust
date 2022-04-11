use log::LevelFilter;
use rbatis::core::db::DBPoolOptions;

use rbatis::plugin::intercept::SqlIntercept;
use rbatis::plugin::log::LogPlugin;


use std::time::Duration;
use futures::executor::block_on;

use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;



lazy_static! {
        pub static ref RB : Rbatis = Rbatis::new();//block_on(   InitDb::new("mysql://root:root123@localhost:3306/test")  );
   }



pub struct InitDb;

impl InitDb {
    pub async fn new(url: &str) -> Rbatis {
        let mut rbatis: Rbatis = Rbatis::new();
        rbatis.add_sql_intercept(Intercept {});
        rbatis.set_log_plugin(RbatisLog {});
        rbatis.link_opt(url, Self::db_option()).await.expect("connect database is error ");
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
        _rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<rbson::Bson>,
        _is_prepared_sql: bool,
    ) -> Result<(), rbatis::core::Error> {
        println!("执行sql: {:?} \n 参数：{:?}", sql, args);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RbatisLog {}

impl LogPlugin for RbatisLog {
    fn get_level_filter(&self) -> &LevelFilter {
        &LevelFilter::Debug
    }
}


async fn process() {
    println!("Hello world!");
}

#[test]
fn etest() {
    let future = process();

    let runtime1 = tokio::runtime::Runtime::new().unwrap();

    let rbatis = runtime1.block_on(InitDb::new("mysql://root:root123@localhost:3306/test"));
    println!("{}", rbatis.is_debug_mode());
}
