use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{copy, AsyncBufReadExt, AsyncWriteExt, BufReader, Error, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel as channel;
use tokio::task::spawn;
use tokio::time::sleep;

#[derive(Debug)]
enum Message {
    Connected(TcpStream),
    Quit,
}

enum ReqResult {
    Ok,
    Quit,
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    let (sx, mut rx) = channel::<Message>();
    let sx1 = sx.clone();
    // 生成线程转发accept
    let _accept_loop = spawn(async move {
        while let Ok((stream, _addr)) = listener.accept().await {
            sx1.send(Message::Connected(stream)).unwrap();
        }
    });
    while let Some(message) = rx.recv().await {
        match message {
            Message::Connected(stream) => {
                // 主进程收到正常发送结果,进行处理
                let sx = sx.clone();
                spawn(async move {
                    if let Ok(ReqResult::Quit) = handle_connection(stream).await {
                        sx.send(Message::Quit).unwrap();
                    }
                });
            }
            Message::Quit => {
                println!("server quit");
                break;
            }
        }
    }
    // accept_loop.await?;
    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<ReqResult> {
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
                    is_file(&req.path).is_some(),
                    is_file(&req.path).unwrap_or("none"),
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
#[derive(Debug)]
struct Request {
    /// 请求类型
    method_type: MethodType,
    // 请求路径
    path: String,
    // 请求条件
    query: Option<HashMap<String, String>>,
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
        Request {
            method_type,
            path,
            query,
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
    if let Some(r#type) = path{
      return r#type.to_str();
    }
    None
}


