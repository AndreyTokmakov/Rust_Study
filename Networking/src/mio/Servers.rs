


mod server_example
{
    use mio::{ net::TcpListener, Events, Interest, Poll, Token };
    use std::collections::HashMap;
    use std::io::{Read, Write};
    use std::net::SocketAddr;
    use mio::net::TcpStream;

    const SERVER: Token = Token(0);

    pub fn runServer() -> std::io::Result<()>
    {
        let mut poll: Poll = Poll::new()?;
        let mut events: Events = Events::with_capacity(128);

        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut listener: TcpListener = TcpListener::bind(addr)?;

        poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

        let mut uniqueToken: usize = 1;
        let mut connTable: HashMap<Token, TcpStream> = HashMap::new();

        println!("Server running on {}", addr);
        loop {
            poll.poll(&mut events, None)?;
            for event in &events {
                match event.token() {
                    SERVER => {
                        let (mut stream, _) = listener.accept()?;
                        let token = Token(uniqueToken);
                        uniqueToken += 1;

                        poll.registry().register(&mut stream, token, Interest::READABLE)?;
                        connTable.insert(token, stream);
                    }
                    token => {
                        if let Some(stream) = connTable.get_mut(&token) {
                            let mut buf: [u8; 1024] = [0; 1024];
                            match stream.read(&mut buf) {
                                Ok(0) => {
                                    connTable.remove(&token);
                                }
                                Ok(n) => {
                                    println!("Read {} bytes", n);
                                    stream.write_all(&buf[..n])?;
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                                Err(e) => return Err(e),
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn test_all()
{
    server_example::runServer().unwrap();
}