
use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;


fn handle_client_1(stream: TcpStream)
{
    println!("New connection from {}", stream.peer_addr().unwrap());
}


fn start_server_1(host: &str, port: u16) -> std::io::Result<()>
{
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client_1(stream?);
    }
    Ok(())
}


fn handle_client_new_thread(mut stream: TcpStream) -> Result<(), Error>
{
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buffer = [0; 512];
    loop {
        let bytesRead = stream.read(&mut buffer)?;
        if bytesRead == 0 {
            return Ok(())
        }
        stream.write(&buffer[..bytesRead])?;
    }
}

fn start_server_multithreaded(host: &str, port: u16)
{
    let listener = TcpListener::bind(format!("{}:{}", host, port)).expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client_new_thread(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

// https://doc.rust-lang.org/std/net/struct.TcpListener.html
pub fn test_all()
{
    // start_server_1("0.0.0.0", 52525).expect("TODO: panic message");
    start_server_multithreaded("0.0.0.0", 52525);
}