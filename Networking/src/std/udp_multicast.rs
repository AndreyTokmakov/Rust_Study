
mod receiver
{
    use std::io;
    use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

    pub fn run() -> io::Result<()>
    {
        let multicast_ip: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
        let port: u16  = 8888;

        // Слушаем на всех интерфейсах
        let bind_addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);

        let socket = UdpSocket::bind(bind_addr)?;

        // Вступаем в multicast-группу
        socket.join_multicast_v4(&multicast_ip, &Ipv4Addr::UNSPECIFIED)?;

        println!("Listening {}:{}", multicast_ip, port);
        let mut buffer = [0u8; 1500];
        loop {
            let (size, sender) = socket.recv_from(&mut buffer)?;
            println!("{} bytes from {} : {}", size, sender, String::from_utf8_lossy(&buffer[..size]));
        }
    }
}

mod producer
{
    use std::{io,
              net::{Ipv4Addr, SocketAddrV4, UdpSocket},
              thread,
              time::Duration,
    };

    pub fn run() -> io::Result<()>
    {
        let multicast: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(239, 255, 0, 1), 8888, );
        let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;

        socket.set_multicast_loop_v4(true)?;
        socket.set_multicast_ttl_v4(1)?;

        let mut counter = 0;
        loop {
            let msg: String = format!("Hello {}", counter);
            socket.send_to(msg.as_bytes(), multicast)?;
            println!("Sent: {}", msg);
            counter += 1;
            thread::sleep(Duration::from_secs(1));
        }
    }
}

pub fn test_all()
{
    receiver::run().unwrap();
    producer::run().unwrap();
}