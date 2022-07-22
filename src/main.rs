use tokio::io::{copy, AsyncBufReadExt, AsyncWriteExt, BufReader, Error, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::unbounded_channel as channel;
use tokio::task::spawn;
mod utils;
use utils::request::Message;
use utils::request::ReqResult;


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
                    if let Ok(ReqResult::Quit) = utils::request::handle_connection(stream).await {
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

