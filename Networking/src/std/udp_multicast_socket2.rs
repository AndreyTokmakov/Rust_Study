
mod receiver
{
    use log::{info, debug};
    use socket2::{Domain, Protocol, Socket, Type};
    use std::io;
    use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

    pub fn run() -> io::Result<()>
    {
        let port: u16  = 8888;
        let group: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
        let socket: Socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        // setsockopt()
        #[cfg(target_family = "unix")]
        socket.set_reuse_port(true)?;

        socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port).into(),)?;
        socket.join_multicast_v4(&group,&Ipv4Addr::UNSPECIFIED,)?;

        let socket: UdpSocket = socket.into();
        info!("Listening {}:{}", group, port);
        let mut buffer: [u8 ; 1024 * 2] = [0u8; 2048];
        loop {
            let (size, sender) = socket.recv_from(&mut buffer)?;
            info!("{} bytes from {} : {}", size, sender, String::from_utf8_lossy(&buffer[..size]),);
        }
    }
}

mod producer
{
    use log::{info, debug};
    use socket2::{Domain, Protocol, Socket, Type};
    use std::{
        io,
        net::{Ipv4Addr, SocketAddrV4, UdpSocket},
        thread,
        time::Duration,
    };

    pub fn run() -> io::Result<()>
    {
        let port: u16  = 8888;
        let address: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 1);
        let socket: Socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        let group: SocketAddrV4 = SocketAddrV4::new(address, port);

        socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0).into(),)?;
        socket.set_multicast_ttl_v4(5)?;
        socket.set_multicast_loop_v4(true)?;

        let socket: UdpSocket = socket.into();
        let mut counter = 0;
        loop {
            let msg: String = format!("Packet {}", counter);
            socket.send_to(msg.as_bytes(), group)?;
            info!("Sent {}", msg);
            counter += 1;
            thread::sleep(Duration::from_secs(1));
        }
    }
}

mod runner
{
    use std::io;
    use std::str::FromStr;
    use crate::udp_multicast_socket2::{producer, receiver};

    enum Mode {
        Receiver,
        Producer,
    }

    impl FromStr for Mode
    {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err>
        {
            match s.to_ascii_lowercase().as_str() {
                "receiver" | "recv" => Ok(Self::Receiver),
                "producer" | "sender" | "send" => Ok(Self::Producer),
                _ => Err("unknown mode"),
            }
        }
    }

    fn demo(mode: Mode) -> io::Result<()> {
        match mode {
            Mode::Receiver => receiver::run(),
            Mode::Producer => producer::run(),
        }
    }

    pub fn run() -> io::Result<()>
    {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .format_timestamp_millis()
            .init();

        let mode: Mode = std::env::args().nth(1).expect("usage: app <receiver|producer>")
            .parse::<Mode>().expect("invalid mode");
        demo(mode)
    }
}

pub fn test_all()
{
    runner::run().expect("runner failed");

    // cargo run -- receiver
    // cargo run -- producer
}