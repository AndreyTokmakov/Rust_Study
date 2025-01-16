
use tokio_tungstenite::accept_async;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;

use url::Url;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Utf8Bytes;
use log::{info, error};

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()>
{
    let mut ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!("Message received: {}", received_text);

            let response = format!("[{}]", received_text);
            ws_stream.send(Message::Text(Utf8Bytes::from(response))).await?;
            println!("Message send: {}", received_text.to_string());
        }
    }
    Ok(())
}

async fn handle_connection_2(stream: TcpStream)
{
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket stream into a sender and receiver
    let (mut sender, mut receiver)
        = ws_stream.split();

    while let Some(msg) = receiver.next().await {
        match msg
        {
            Ok(Message::Text(text)) =>
            {
                println!("Message received: {}", &text);
                // Reverse the received string and send it back
                let response: String = format!("[{}]", text);
                if let Err(e) = sender.send(Message::Text(Utf8Bytes::from(&response))).await {
                    error!("Error sending message: {}", e);
                }

                println!("Message send: {}", &response.to_string());
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn server_main() -> Result<()>
{
    let addr: String = "127.0.0.1:6789".to_string();
    let listener: TcpListener = TcpListener::bind(&addr).await.
        expect(format!("Failed to bind to {addr}").as_str());

    println!("WebSocket server started on ws://{}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
        // tokio::spawn(handle_connection_2(stream));
    }

    Ok(())
}


#[tokio::main]
async fn client_main() -> Result<()>
{
    let url = Url::parse("ws://127.0.0.1:6789   ")?;
    let (mut ws_stream, _) = connect_async(url.as_str()).await.expect("Failed to connect");
    println!("WebSocket client connected");

    // Sending a message to the server
    let message = "Hello, Server!";
    ws_stream.send(Message::Text(message.into())).await?;

    // Receiving messages from the server
    while let Some(msg) = ws_stream.next().await {
        match msg? {
            Message::Text(text) => {
                println!("Received message from server: {}", text);
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn test_all()
{
    server_main();
}