use rustdns::Message;
use rustdns::types::*;
use std::net::UdpSocket;
use std::time::Duration;

/*
fn test_1() -> Result<(), Box<dyn std::error::Error>>
{
    // A DNS Message can be easily constructed
    let mut msg: Message = Message::default();
    msg.add_question("bramp.net", Type::A, Class::Internet);

    msg.add_extension(Extension {  // Optionally add a EDNS extension
        payload_size: 4096,            // which supports a larger payload size.
        ..Default::default()
    });

    // Setup a UDP socket for sending to a DNS server.
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    // socket.set_read_timeout(Some(Duration::new(5, 0)))?;
    // socket.connect("8.8.8.8:53")?; // Google's Public DNS Servers
}
*/

pub fn test_all()
{

}
