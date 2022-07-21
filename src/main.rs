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

enum ReqResult{
  Ok,
  Quit,
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    let (sx,mut rx) = channel::<Message>();
    let sx1 = sx.clone();
    // 生成线程转发accept
    let accept_loop = spawn(async move {
      while let Ok((stream,_addr)) = listener.accept().await{
        sx1.send(Message::Connected(stream)).unwrap();
      }
    });
    while let Some(message) = rx.recv().await{
      match message{
        Message::Connected(stream) => {
          // 主进程收到正常发送结果,进行处理
          let sx = sx.clone();
          spawn(async move{
            if let Ok(ReqResult::Quit) = handle_connection(stream).await {
              sx.send(Message::Quit).unwrap();
            }
          });
        },
        Message::Quit=>{
          println!("server quit");
          break;
        }
      }
    }
    accept_loop.await?;
    todo!()
}


async fn handle_connection(mut stream: TcpStream)->Result<ReqResult>{
  Ok(ReqResult::Ok)
}