use domain::{common::res::Resp, system::auth::LoginResponse};

/// 用户登录

pub async fn login() -> Resp<LoginResponse> {
    Resp::new(
        true,
        Some(LoginResponse::default()),
        0,
        "登录成功".to_string(),
        2,
        "traceId".to_string(),
    )
}
