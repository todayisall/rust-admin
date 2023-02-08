use axum::Router;

use super::common;

// 注册路由
pub fn register_router() -> Router {
    // 创建路由
    Router::new()
        // 注册系统路由
        // .nest("/system", system::register_router())
        // 注册通用路由
        .nest("/common", common::register_no_auth_routes())
}
