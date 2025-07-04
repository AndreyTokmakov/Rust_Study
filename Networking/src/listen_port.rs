
use std::net::TcpListener;
use std::io::{Read, Error};


// In this example, the port is displayed on the console, and the program will listen until a request is made. 
// TcpListener::bind uses a random port allocated by the OS when requested to bind to port 0.
fn listen_port_TCP() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:52525")?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}

pub fn test_all()
{
    listen_port_TCP().expect("Failed to listen port");
}