
mod clients
{
    use std::thread;
    use std::thread::sleep;
    use zmq::{Context, Socket};
    use std::time::Duration;

    pub fn req_client()
    {
        let context: Context = zmq::Context::new();
        let socket: Socket =  context.socket(zmq::REQ).unwrap();
        socket.connect("tcp://localhost:5556").unwrap();

        for i in 0..10 {
            let request = format!("Hello {}", i);
            println!("Sending: {}", request);
            socket.send(&request, 0).unwrap();

            let reply = socket.recv_string(0).unwrap().unwrap();
            println!("Received reply: {}", reply);

            thread::sleep(Duration::from_millis(250));
        }
    }
}

pub fn test_all()
{
    clients::req_client();
}