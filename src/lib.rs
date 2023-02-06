#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

//配置
pub mod config;
//初始化
pub mod init;
//api
pub mod apis;
//services
pub mod services;
//utils
pub mod utils;
//models
pub mod models;
//controller
pub mod controllers;

use log::info;
use state::Container;

use config::config::ApplicationConfig;
use init::init_config;
use init::init_log;
use init::init_service;

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

/*初始化环境上下文*/
pub async fn init_context() {
    print_banner();
    //第一步加载配置
    init_config().await;
    //第二步加载日志
    init_log();
    info!("ConfigContext init complete");
    //第三步初始化cache
    init_service().await;
    let _config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    info!(" - Local:   http://{}:{}", _config.server().host().replace("0.0.0.0", "127.0.0.1"), _config.server().port());
}

fn print_banner() {
    let banner = r#"
     ____
    |      。   ———————     |                |     |        _____    ____
    |___   |   |            |                |     |       |        |
    |      |   |_______     |         —————— |     |       |_____   |____
    |      |   |            |         |      |     |       |        |
    |      |   |————————    |______   |_____ |     |____   |_____   |____
"#;
    println!("{}", banner);
}
