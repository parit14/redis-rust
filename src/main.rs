// use std::{io::{Write, Read}, net::TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut stream, _addr)) => {
                tokio::spawn(async move {
                    let mut buf = [0; 512];
                    loop {
                        let read_count = stream.read(&mut buf).await.unwrap();
                        if read_count == 0 {
                            break;
                        }
                        stream.write("+PONG\r\n".as_bytes()).await.unwrap();
                    }
                    println!("accepted new connection");
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
