#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub database: _Config__database,
    pub log: _Config__log,
    pub whitelist: _Config__whitelist,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct _Config__database {
    pub password: Cow<'static, str>,
    pub url: Cow<'static, str>,
    pub username: Cow<'static, str>,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct _Config__log {
    pub log_dir: Cow<'static, str>,
    pub log_level: Cow<'static, str>,
    pub log_pack_compress: Cow<'static, str>,
    pub log_rolling_type: Cow<'static, str>,
    pub log_temp_size: Cow<'static, str>,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct _Config__whitelist {
    pub list: Cow<'static, [Cow<'static, str>]>,
}

pub const CONFIG: Config = Config {
    database: _Config__database {
        password: Cow::Borrowed(""),
        url: Cow::Borrowed(""),
        username: Cow::Borrowed(""),
    },
    log: _Config__log {
        log_dir: Cow::Borrowed("target/logs/"),
        log_level: Cow::Borrowed("info"),
        log_pack_compress: Cow::Borrowed(""),
        log_rolling_type: Cow::Borrowed("KeepNum(20)"),
        log_temp_size: Cow::Borrowed("100MB"),
    },
    whitelist: _Config__whitelist {
        list: Cow::Borrowed(&[Cow::Borrowed("/admin/index/send_reg_code"), Cow::Borrowed("/admin/index/login")]),
    },
};
