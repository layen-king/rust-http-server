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
    let req = process_request(&str);
    println!("req :{:?}", req);
    // 判断请求类型
    match req.method {
        Method::GET => {
            // 判断是否为quit,若为quit,返回退出
            if req.path == "/quit" {
                return Ok(ReqResult::Quit);
            } else {
                let context = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    req.path.len(),
                    req.path
                );
                stream.write(context.as_bytes()).await?;
            }
            stream.flush().await?;
        }
        Method::POST => {}
    }
    Ok(ReqResult::Ok)
}
#[derive(Debug)]
enum Method {
    GET,
    POST,
}
#[derive(Debug)]
struct Request {
    method: Method,
    path: String,
}

impl Request {
    fn new(req: &str) -> Request {
        let v = req.split_whitespace().collect::<Vec<_>>();
        let mut method = Method::GET;
        if let Some(m) = v.get(0) {
            match m {
                &"GET" => method = Method::GET,
                &"POST" => method = Method::POST,
                _ => method = Method::GET,
            }
        };
        Request {
            method,
            path: v.get(1).unwrap_or(&"").to_string(),
        }
    }
}

/// 处理请求
/// ### [req] 请求的指针
fn process_request(req: &str) -> Request {
    Request::new(req)
}
