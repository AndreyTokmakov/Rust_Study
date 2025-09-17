use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn send_request() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Send a message to the server
    stream.write_all(b"Hello, server!").await?;

    // Read the response from the server
    let mut buffer = [0; 1024];
    let n: usize = stream.read(&mut buffer).await?;

    // Print the response
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}


// https://softwarepatternslexicon.com/patterns-rust/12/1/
pub fn test_all()
{
    let _ = send_request();
}

