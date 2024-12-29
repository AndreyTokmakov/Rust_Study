
use std::net::UdpSocket;

fn run_server(host: &str, port: u16) -> std::io::Result<()>
{
    {
        let socket = UdpSocket::bind(format!("{}:{}", host, port))?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold the message, it will be cut off.
        let mut buffer = [0; 512];
        let (bytes, src) = socket.recv_from(&mut buffer)?;
        let payload = str::from_utf8(buffer[..bytes]).unwrap();

        println!("Received {} bytes from {}: {:?}", bytes, src, payload);
    } // the socket is closed here
    Ok(())
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

pub fn test_all()
{
    run_server("0.0.0.0", 52525).expect("TODO: panic message");
    // run_server_echo("0.0.0.0", 52525);
}