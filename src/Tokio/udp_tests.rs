
use tokio::{net::UdpSocket, sync::mpsc};
use std::{io, net::SocketAddr, sync::Arc};


#[tokio::main]
async fn start_udp_server_synch() -> io::Result<()>
{
    let sock = UdpSocket::bind("0.0.0.0:52525").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
}

#[tokio::main]
async fn start_udp_server_asynch() -> io::Result<()>
{
    let sock = UdpSocket::bind("0.0.0.0:52525".parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(sock);
    let s = r.clone();
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            let len = s.send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len);
        }
    });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}

// https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html
pub fn test_all()
{
    start_udp_server_synch();
    // start_udp_server_asynch();
}