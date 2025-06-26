extern crate websocket;
extern crate native_tls;

use std::sync::mpsc::channel;
use std::thread;
use std::io::{stdin, Read};
use std::fs::File;
use websocket::{ClientBuilder, Message, OwnedMessage};
use websocket::sync::Server;
use native_tls::{Identity, TlsAcceptor};
use tokio::net::TcpStream;

const URL: &'static str = "wss://stream.binance.com/stream";

pub fn test_all()
{
    println!("Connecting to {}", URL);

    let client = ClientBuilder::new(URL).unwrap()
        .add_protocol("rust-websocket").connect_insecure().unwrap();

    println!("Successfully connected");

    let (mut receiver, mut sender) = client.split().unwrap();
    let (tx, rx) = channel();
    let tx_1 = tx.clone();

    let send_loop = thread::spawn(move || {
        loop {
            // Send loop
            let message = match rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    return;
                }
                _ => (),
            }
            // Send the message
            match sender.send_message(&message) {
                Ok(()) => (),
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    let _ = sender.send_message(&Message::close());
                    return;
                }
            }
        }
    });

    let receive_loop = thread::spawn(move || {
        // Receive loop
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx_1.send(OwnedMessage::Close(None));
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = tx_1.send(OwnedMessage::Close(None));
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match tx_1.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);
                            return;
                        }
                    }
                }
                // Say what we received
                _ => println!("Receive Loop: {:?}", message),
            }
        }
    });


    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let trimmed: &str = input.trim();

        let message = match trimmed {
            "/close" => {
                // Close the connection
                let _ = tx.send(OwnedMessage::Close(None));
                break;
            }
            // Send a ping
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
            // Otherwise, just send text
            _ => OwnedMessage::Text(trimmed.to_string()),
        };

        match tx.send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e);
                break;
            }
        }
    }

    // We're exiting

    println!("Waiting for child threads to exit");
    let _ = send_loop.join();
    let _ = receive_loop.join();
    println!("Exited");
}   