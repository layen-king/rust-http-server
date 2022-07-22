use std::collections::HashMap;
use std::path::Path;
#[allow(unused_imports)]
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{copy, AsyncBufReadExt, AsyncWriteExt, BufReader, Error, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel as channel;
use tokio::task::spawn;
use tokio::time::sleep;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Message {
    Connected(TcpStream),
    Quit,
}

pub enum ReqResult {
    Ok,
    Quit,
}

/// 流处理
pub async fn handle_connection(mut stream: TcpStream) -> Result<ReqResult> {
    let mut str = String::new();
    BufReader::new(&mut stream).read_line(&mut str).await?;
    let req = Request::new(&str);
    // 判断请求类型
    match req.method_type {
        MethodType::GET => {
            // todo:get请求判断是否为静态文件 分别处理
            // 判断是否为quit,若为quit,返回退出
            if req.path == "/quit" {
                return Ok(ReqResult::Quit);
            } else {
                let mut query_str = String::new();
                if let Some(q) = req.query {
                    query_str.push_str(&format!("{:?}", q));
                }
                let output = format!(
                    "path:{} \r\nis file:{},   file type:{} \r\nquery:{}   \r\ntime:{:?}",
                    req.path,
                    req.is_file,
                    req.file_type,
                    query_str,
                    std::time::SystemTime::now()
                );
                let context = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    output.len(),
                    output
                );
                stream.write(context.as_bytes()).await?;
            }
            stream.flush().await?;
        }
        MethodType::POST => {}
    }
    Ok(ReqResult::Ok)
}

/// 请求头类型
#[derive(Debug)]
enum MethodType {
    GET,
    POST,
}

/// 请求头
#[derive(Debug)]
struct Request {
    /// 请求类型
    method_type: MethodType,
    // 请求路径
    path: String,
    // 请求条件
    query: Option<HashMap<String, String>>,
    // 是否为文件
    is_file: bool,
    // 文件类型
    file_type: String,
}

impl Request {
    fn new(req: &str) -> Request {
        let v = req.split_whitespace().collect::<Vec<_>>();
        let mut method_type = MethodType::GET;
        // 生成method_type
        if let Some(m) = v.get(0) {
            match m {
                &"GET" => method_type = MethodType::GET,
                &"POST" => method_type = MethodType::POST,
                _ => method_type = MethodType::GET,
            }
        };
        let mut path = String::new();
        let mut query: Option<_> = None;
        if let Some(p) = v.get(1) {
            // 存在? 分割成path和query
            if p.contains("?") {
                let v = p.split_once("?").unwrap();
                path.push_str(v.0);
                query = process_request_query(v.1);
            } else {
                path.push_str(v.get(1).unwrap());
            }
        }
        let path_is_file = is_file(&path).is_some();
        let file_type = is_file(&path).unwrap_or("").to_string();
        Request {
            method_type,
            path,
            query,
            is_file:path_is_file,
            file_type
        }
    }
}

/// 将path分割成query
fn process_request_query(query: &str) -> Option<HashMap<String, String>> {
    // 去除头尾的&
    let query = query.trim_matches('&');
    // 使用&分割
    let kv = query.split("&").collect::<Vec<&str>>();
    // 转换成hashMap
    let mut hash_map = HashMap::new();
    for str in kv.iter() {
        if str.contains("=") {
            let kv = str.split_once("=").unwrap();
            hash_map.insert(kv.0.to_string(), kv.1.to_string());
        } else {
            continue;
        }
    }
    if hash_map.is_empty() {
        return None;
    }
    Some(hash_map)
}

/// 判断请求路径是否为文件
/// [path] 请求路径
fn is_file(path: &str) -> Option<&str> {
    let path = Path::new(path).extension();
    if let Some(r#type) = path {
        return r#type.to_str();
    }
    None
}
