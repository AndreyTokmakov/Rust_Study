
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

fn exec_ssh_command() -> Result<(), Box<dyn std::error::Error>>
{
    let tcp: TcpStream = TcpStream::connect("127.0.0.1:22022")?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.userauth_password("test", "test")?;
    assert!(session.authenticated());

    let mut channel = session.channel_session()?;
    channel.exec("ls -lar")?;

    let mut buffer: String = String::new();
    channel.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    channel.wait_close()?;
    Ok(())
}

pub fn test_all()
{
    exec_ssh_command().expect("TODO: panic message");
}