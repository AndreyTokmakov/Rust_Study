
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
    
    pub fn start(host: &str, port: u16) -> std::io::Result<()>
    {
        let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port))?;
        for stream in listener.incoming() {
            handle_client_1(stream?);
        }
        Ok(())
    }
}


mod echo_server
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
    
    pub fn start(host: &str, port: u16) -> std::io::Result<()>
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


mod multithreaded_echo_server
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

    pub fn start(host: &str, port: u16)
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

mod non_blocking_echo_server
{
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::time::Duration;
    use std::thread;

    pub fn start(host: &str, port: u16) -> std::io::Result<()>
    {
        let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port)).expect("Could not bind");
        listener.set_nonblocking(true)?;

        let mut clients: Vec<TcpStream> = Vec::new();
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("New client: {}", addr);
                    stream.set_nonblocking(true)?;
                    clients.push(stream);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // no new client right now
                }
                Err(e) => return Err(e),
            }

            clients.retain_mut(|stream| {
                let mut buf = [0; 512];
                match stream.read(&mut buf) {
                    Ok(0) => false, // client disconnected
                    Ok(n) => {
                        let _ = stream.write_all(&buf[..n]);
                        true
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => true,
                    Err(_) => false,
                }
            });

            thread::sleep(Duration::from_millis(100));
        }
    }
}

mod non_blocking_mio_server
{
    use std::collections::HashMap;
    use mio::net::{TcpListener, TcpStream};
    use mio::{Events, Interest, Poll, Token};
    use std::io::{Read, Write};
    use std::net::SocketAddr;
    use std::time::Duration;

    const SERVER: Token = Token(0);

    pub fn start(host: &str, port: u16) -> std::io::Result<()>
    {
        let mut poll: Poll = Poll::new()?;
        let mut events: Events = Events::with_capacity(128);
        let addr: SocketAddr = format!("{host}:{port}").parse().unwrap();
        let mut server = TcpListener::bind(addr)?;

        poll.registry().register(&mut server, SERVER, Interest::READABLE)?;
        let mut unique_token: usize = 1;
        let mut connTable: HashMap<Token, TcpStream> = std::collections::HashMap::new();

        loop {
            poll.poll(&mut events, Some(Duration::from_millis(100)))?;

            for event in &events {
                match event.token() {
                    SERVER => {
                        let (mut stream, addr) = server.accept()?;
                        println!("New client: {}", addr);
                        let token = Token(unique_token);
                        unique_token += 1;

                        poll.registry()
                            .register(&mut stream, token, Interest::READABLE | Interest::WRITABLE)?;

                        connTable.insert(token, stream);
                    }
                    token => {
                        if let Some(stream) = connTable.get_mut(&token) {
                            let mut buf = [0; 512];
                            match stream.read(&mut buf) {
                                Ok(0) => {
                                    println!("Client disconnected");
                                    connTable.remove(&token);
                                }
                                Ok(n) => {
                                    let msg = String::from_utf8_lossy(&buf[..n]);
                                    println!("Received: {}", msg);
                                    let _ = stream.write_all(b"Echo from mio\n");
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                                Err(_) => {
                                    connTable.remove(&token);
                                }
                            }
                        }
                    }
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

    // example_1::start(&host, port).expect("TODO: panic message");
    // echo_server:: start(&host, port).expect("TODO: panic message");
    // multithreaded_echo_server::start(&host, port);
    non_blocking_echo_server::start(&host, port).expect("TODO: panic message");
}