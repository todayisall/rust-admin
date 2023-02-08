use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("图片验证码错误")]
    CaptchaError(String), // 验证码错误
    #[error("账号或密码错误")]
    UsernameOrPasswordError(String), // 账号或密码错误
    #[error("账户被禁用")]
    AccountDisabled(String), // 用户被禁用

    #[error("登录失败次数超过5次")]
    LoginFailedLimit(String), // 登录失败次数过多
    #[error("账户被锁定")]
    AccountLocked(String), // 账户被锁定
}

// 登录类型
#[derive(Serialize, Deserialize)]
pub enum LoginType {
    Account,
    Phone,
    Email,
}

// 登录请求
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    login_type: LoginType,
    username: String,
    password: String,
    captcha: String,
}

// 登录响应
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
}

impl Default for LoginResponse {
    fn default() -> Self {
        LoginResponse {
            token: "".to_string(),
        }
    }
}
