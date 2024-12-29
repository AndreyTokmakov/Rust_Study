
use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream, Shutdown};
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


fn handle_client_new_thread(mut stream: TcpStream)
{
    let mut buffer = [0; 512];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            // echo everything!
            stream.write(&buffer[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn start_echo_server_multithreaded(host: &str, port: u16)
{
    let listener = TcpListener::bind(format!("{}:{}", host, port)).expect("Could not bind");
    for stream in listener.incoming()
    {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || { handle_client_new_thread(stream); });
            }
            Err(error) => {
                eprintln!("failed: {}", error)
                /* connection failed */
            }
        }
    }
}

// https://doc.rust-lang.org/std/net/struct.TcpListener.html
pub fn test_all()
{
    // start_server_1("0.0.0.0", 52525).expect("TODO: panic message");
    start_echo_server_multithreaded("0.0.0.0", 52525);
}