

mod multithreaded_client_server
{
    use std::io;
    use std::io::prelude::*;
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::time::Duration;

    fn one_response(mut socket: TcpStream, n: u64) -> io::Result<()>
    {   // Using format! instead of write! avoids breaking up lines across multiple writes.
        // This is easier than doing line buffering on the client side.
        let start_msg: String = format!("start {n}\n");
        socket.write_all(start_msg.as_bytes())?;
        thread::sleep(Duration::from_secs(1));
        let end_msg: String = format!("end {n}\n");
        socket.write_all(end_msg.as_bytes())?;
        Ok(())
    }

    fn server_main(listener: TcpListener) -> io::Result<()>
    {
        let mut n = 1;
        loop {
            let (socket, _) = listener.accept()?;
            thread::spawn(move || one_response(socket, n).unwrap());
            n += 1;
        }
    }

    fn client_main() -> io::Result<()>
    {
        let mut socket = TcpStream::connect("localhost:8000")?;
        io::copy(&mut socket, &mut io::stdout())?;
        Ok(())
    }

    pub fn run() -> io::Result<()>
    {
        // Avoid a race between bind and connect by binding before spawn.
        let listener: TcpListener = TcpListener::bind("0.0.0.0:8000")?;
        // Start the server on a background thread.
        thread::spawn(|| server_main(listener).unwrap());
        // Run ten clients on ten different threads.
        let mut client_handles = Vec::new();
        for _ in 1..=10 {
            client_handles.push(thread::spawn(client_main));
        }
        for handle in client_handles {
            handle.join().unwrap()?;
        }
        Ok(())
    }
}

pub fn test_all()
{
    multithreaded_client_server::run().expect("TODO: panic message");
}