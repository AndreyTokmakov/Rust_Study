use std::env;
use std::error::Error;
use clap::builder::Str;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn run_echo_server() -> Result<(), Box<dyn Error>>
{
    // Allow passing an address to listen on as the first argument of this  program, but otherwise 
    // we'll just set up our TCP listener on 127.0.0.1:8080 for connections.
    let addr: String = env::args().nth(1)
        .unwrap_or_else(|| "0.0.0.0:52525".to_string());
    
    let listener: TcpListener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer: Vec<u8> = vec![0; 1024];
            // In a loop, read data from the socket and write the data back.
            loop {
                let bytesRead: usize = socket.read(&mut buffer).await
                    .expect("failed to read data from socket");
                if bytesRead == 0 {
                    return;
                }
                socket.write_all(&buffer[0 .. bytesRead]).await
                    .expect("failed to write data to socket");
            }
        });
    }
}

pub fn test_all()
{
    let res = run_echo_server();
}