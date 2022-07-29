use std::collections::HashMap;

trait HashHead {
    fn write(&self) -> String {
        String::new()
    }
}

impl HashHead for HashMap<String, String> {
    fn write(&self) -> String {
        let mut head = String::new();
        for (key, value) in self.iter() {
            head.push_str(&format!("{}:{}\r\n", key, value));
        }
        head
    }
}

/// 响应状态
pub struct ResStatus {
    status_code: u32,
    version: String,
}

impl ResStatus {
    fn new(status_code: u32, version: String) -> ResStatus {
        ResStatus {
            status_code,
            version,
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
    /// 设置响应头
    /// [headers] 响应头属性
    pub fn set_headers(&mut self, k: &str, v: &str) {
        self.headers.insert(k.to_string(), v.to_string());
    }
    /// 设置响应正文,可多次调用添加
    /// [context] 响应正文
    pub fn set_body(&mut self, context: String) {
        self.context.push_str(&context);
    }
    /// 生成文本内容
    pub fn gen_context(&mut self) -> String {
        let mut context = String::new();
        context.push_str(&format!(
            "{} {}\r\n",
            self.status.version, self.status.status_code
        ));
        // 响应头添加正文长度
        self.set_headers(
            "Content-Length",
            &format!("{}", self.context.len()),
        );
        // 写入响应头
        context.push_str(&self.headers.write());
        // 写入空行
        context.push_str("\r\n");
        // 写入正文
        context.push_str(&self.context);
        println!("context: \r\n {}", &context);
        context
    }
}
