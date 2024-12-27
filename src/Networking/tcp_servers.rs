
use std::net::{TcpListener, TcpStream};


fn handle_client(stream: TcpStream)
{
    println!("New connection from {}", stream.peer_addr().unwrap());
}


fn start_server(host: &str, port: u16) -> std::io::Result<()>
{
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

// https://doc.rust-lang.org/std/net/struct.TcpListener.html
pub fn test_all()
{
    start_server("0.0.0.0", 52525).expect("TODO: panic message");
}