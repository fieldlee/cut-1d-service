use axum::{http::Uri, response::IntoResponse, Router, Server};
use cut_1d_service::{apis, config::config::ApplicationConfig, init_context, utils::RespVO, APPLICATION_CONTEXT};
use log::warn;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

async fn fallback(uri: Uri) -> impl IntoResponse {
    let msg = format!("地址不存在：{}", uri);
    warn!("{}", msg.clone());
    RespVO::<String> {
        code: Some(-1),
        msg: Some(msg),
        data: None,
    }
    .resp_json()
}

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let commerce_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let server = format!("{}:{}", commerce_config.server().host(), commerce_config.server().port());
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any).max_age(Duration::from_secs(60) * 10);
    //绑定端口 初始化 路由
    let app = Router::new()
        //  .nest("/admin", admin::routers())
        .nest("/api", apis::api::routers())
        .layer(cors)
        .fallback(fallback);
    // 启动服务
    Server::bind(&server.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}
