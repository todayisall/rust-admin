use axum::{routing::post, Router};

use crate::system;

// 注册无需权限校验的路由
pub fn register_no_auth_routes() -> Router {
    Router::new().route("/login", post(system::login).get(system::login)) // 登录
                                                                          // .route("/get_captcha", get(system::get_captcha)) // 获取验证码
                                                                          // .route("/logout", post(system::log_out)) // 退出登录
}
