
use std::net::UdpSocket;
use std::str;

fn run_server(host: &str, port: u16) -> std::io::Result<()>
{
    {
        let socket = UdpSocket::bind(format!("{}:{}", host, port))?;
        let mut buffer = [0; 512];
        loop {
            let (bytes, src) = socket.recv_from(&mut buffer)?;
            let payload = str::from_utf8(&buffer[..bytes]).unwrap();
            println!("Received {} bytes from {}: {:?}", bytes, src, payload);
        }
    } // the socket is closed here
}

fn run_server_echo(host: &str, port: u16) -> std::io::Result<()>
{
    {
        let socket = UdpSocket::bind(format!("{}:{}", host, port))?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 512];
        let (amt, src) = socket.recv_from(&mut buf)?;

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    } // the socket is closed here
    Ok(())
}

// https://jan.newmarch.name/NetworkProgramming/UDP/wrapper.html?rust

// https://softwarepatternslexicon.com/patterns-rust/12/2/
pub fn test_all()
{
    run_server("0.0.0.0", 52525).expect("TODO: panic message");
    // run_server_echo("0.0.0.0", 52525);
}