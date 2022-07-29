use std::collections::HashMap;

/// 响应状态
pub struct ResStatus {
    status_code: u32,
    version: String,
}

impl ResStatus {
    fn new(status_code: u32, version: String) -> ResStatus {
        ResStatus {
            status_code,
            version: String::from(version),
        }
    }
}

/// http返回结构体
pub struct Response {
    /// 响应状态
    status: ResStatus,
    /// 响应头
    headers: HashMap<String, String>,
    /// 响应正文
    context: String,
}

impl Response {
    pub fn new(status_code: u32, version: String) -> Response {
        Response {
            status: ResStatus::new(status_code, version),
            headers: HashMap::new(),
            context: String::new(),
        }
    }
}
