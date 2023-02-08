use axum::{
    body::{self, Full},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// 请求相公的公共结构体
// 公共返回结构体
#[derive(Debug, Serialize, Default)]
pub struct Resp<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error_code: u32,
    pub error_message: String,
    pub show_type: u32,
    pub trace_id: String,
}

impl<T> Resp<T> {
    pub fn new(
        success: bool,
        data: Option<T>,
        error_code: u32,
        error_message: String,
        show_type: u32,
        trace_id: String,
    ) -> Self {
        Resp {
            success,
            data,
            error_code,
            error_message,
            show_type,
            trace_id,
        }
    }
}

// 为 Resp 实现 axum 的 IntoResponse trait
impl<T> IntoResponse for Resp<T>
where
    T: Serialize + Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            success: self.success,
            data: self.data,
            error_code: self.error_code,
            error_message: self.error_message,
            show_type: self.show_type,
            trace_id: self.trace_id,
        };

        let json_string = match serde_json::to_string(&data) {
            Ok(json_string) => json_string,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(body::boxed(Full::from(e.to_string())))
                    .unwrap();
            }
        };
        let resp_json_string = json_string.clone();
        let mut response = json_string.into_response();
        response.extensions_mut().insert(resp_json_string);
        response
    }
}

// 分页结构体
pub struct Page<T> {
    pub list: Vec<T>,
    pub current: u32,
    pub page_size: u32,
    pub total: u32,
}
