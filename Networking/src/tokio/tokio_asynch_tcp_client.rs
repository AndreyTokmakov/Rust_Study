
mod tcp_client_async_1
{
    use tokio::net::TcpStream;
    use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

    #[tokio::main]
    async fn send_request() -> io::Result<()>
    {
        // Connect to the server
        let mut stream: TcpStream = TcpStream::connect("127.0.0.1:52525").await?;

        // Send a message to the server
        stream.write_all(b"ping").await?;

        // Read the response from the server
        let mut buffer = [0; 1024];
        let n: usize = stream.read(&mut buffer).await?;

        // Print the response
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
        Ok(())
    }

    pub fn run() {
        let _ = send_request();
    }
}


mod http_request
{
    use reqwest::Error;
    use anyhow::Result;

    async fn fetch_url(endpoint: &str) -> Result<String, Error>
    {
        let response = reqwest::get(endpoint).await?;
        println!("Status: {} \nHeaders:\n{:#?}", response.status(), response.headers());

        let body: String = response.text().await?;
        Ok(body)
    }

    #[tokio::main]
    pub async fn call() -> Result<(), Error>
    {
        let endpoint: &str = "https://httpbin.org/get";
        let response: String = fetch_url(endpoint).await?;
        println!("Response:\n{}", response);
        Ok(())
    }

}

// https://softwarepatternslexicon.com/patterns-rust/12/1/
pub fn test_all()
{
    // tcp_client_async_1::run();
    let _  = http_request::call();
}

