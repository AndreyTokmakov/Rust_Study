
use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::{env, thread};


mod example_1
{
    use std::net::{TcpListener, TcpStream};

    fn handle_client_1(stream: TcpStream)
    {
        println!("New connection from {}", stream.peer_addr().unwrap());
    }
    
    pub fn start_server_1(host: &str, port: u16) -> std::io::Result<()>
    {
        let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port))?;
        // accept connections and process them serially
        for stream in listener.incoming() {
            handle_client_1(stream?);
        }
        Ok(())
    }
}


mod example_2
{
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::{Shutdown, TcpListener, TcpStream};

    fn handle_client(mut stream: TcpStream)
    {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer) {
                Ok(bytesRead) => {
                    if bytesRead == 0 {  // connection was closed
                        break;
                    } else { 
                        println!("Received {} bytes: {:?}", bytesRead, std::str::from_utf8(&buffer[..bytesRead]));
                    }
                    stream.write(&buffer[0.. bytesRead]).unwrap();
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
    }
    
    pub fn start_echo_server_simple(host: &str, port: u16) -> std::io::Result<()>
    {
        let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port))?;
        for stream in listener.incoming() {
            let stream: TcpStream = stream?;
            println!("Client connected: {:?}", stream.peer_addr()?);
            handle_client(stream);
        }
        Ok(())
    }
}


mod example_3
{
    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpListener, TcpStream};
    use std::thread;

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

    pub fn start_echo_server_multithreaded(host: &str, port: u16)
    {
        let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port)).expect("Could not bind");
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

}


// https://doc.rust-lang.org/std/net/struct.TcpListener.html
pub fn test_all()
{
    let host: String = env::args().nth(1).unwrap_or_else(|| "0.0.0.0".to_string());
    let port: u16 = env::args().nth(2).unwrap_or_else(|| "52525".to_string()).parse().unwrap_or(52525);

    println!("Running on: {}:{}", host, port);

    // example_1::start_server_1(&host, port).expect("TODO: panic message");
    example_2:: start_echo_server_simple(&host, port).expect("TODO: panic message");
    // example_3:: start_echo_server_multithreaded(&host, port);
}