use tokio::net::TcpListener;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn run_server() -> io::Result<()>
{
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        // Accept an incoming connection
        let (mut socket, _) = listener.accept().await?;

        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            // Read data from the socket
            match socket.read(&mut buffer).await {
                Ok(n) if n == 0 => return, // Connection closed
                Ok(n) => {
                    // Echo the data back to the client
                    if let Err(e) = socket.write_all(&buffer[..n]).await {
                        eprintln!("Failed to write to socket; err = {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket; err = {:?}", e);
                }
            }
        });
    }
}

pub fn test_all()
{
    let _ = run_server();
}

