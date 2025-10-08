
mod rep_req
{
    use std::thread;
    use std::thread::sleep;
    use zmq::{Context, Socket};
    use std::time::Duration;

    fn client()
    {
        let context: Context = zmq::Context::new();
        let socket: Socket =  context.socket(zmq::REQ).unwrap();
        socket.connect("tcp://localhost:5555").unwrap();

        for i in 0..3 {
            let request = format!("Hello {}", i);
            println!("Sending: {}", request);
            socket.send(&request, 0).unwrap();

            let reply = socket.recv_string(0).unwrap().unwrap();
            println!("Received reply: {}", reply);

            thread::sleep(Duration::from_secs(1));
        }
    }

    fn server()
    {
        let context: Context = zmq::Context::new();
        let socket: Socket = context.socket(zmq::REP).unwrap();
        socket.bind("tcp://*:5555").unwrap();

        println!("Server listening on tcp://*:5555");
        loop {
            // Receive request
            let msg: String = socket.recv_string(0).unwrap().unwrap();
            println!("Received request: {}", msg);

            // Respond to client
            socket.send("World", 0).unwrap();
        }
    }

    pub fn run()
    {
        let client_thread = thread::spawn(|| { client();});
        let server_thread = thread::spawn(|| { server(); });

        client_thread.join().unwrap();
        server_thread.join().unwrap();
    }

    // Sending: Hello 0
    // Server listening on tcp://*:5555
    // Received request: Hello 0
    // Received reply: World
    // Sending: Hello 1
    // Received request: Hello 1
    // Received reply: World
    // Sending: Hello 2
    // Received request: Hello 2
    // Received reply: World
}

mod pub_sub
{
    use std::thread;
    use std::thread::sleep;
    use zmq::{Context, Socket};
    use std::time::Duration;

    static PORT: i32 = 5666;

    fn publisher()
    {
        let context: Context = zmq::Context::new();
        let publisher: Socket = context.socket(zmq::PUB).unwrap();
        publisher.bind(format!("tcp://*:{PORT}").as_str()).unwrap();

        println!("Publisher on tcp://*:{}", PORT);

        for i in 0..10 {
            let msg: String = format!("topic1 Message {}", i);
            publisher.send(&msg, 0).unwrap();
            println!("Sent: {}", msg);
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn subscriber()
    {
        let context: Context = zmq::Context::new();
        let subscriber: Socket = context.socket(zmq::SUB).unwrap();
        subscriber.connect(format!("tcp://localhost:{PORT}").as_str()).unwrap();

        // Subscribe to all messages starting with "topic1"
        subscriber.set_subscribe(b"topic1").unwrap();

        loop {
            let msg: String = subscriber.recv_string(0).unwrap().unwrap();
            println!("Subscriber received: {}", msg);
        }
    }

    pub fn run()
    {
        thread::spawn(|| { publisher(); }).join().unwrap();
        thread::spawn(|| { subscriber(); }).join().unwrap();
    }

    // Publisher on tcp://*:5556
    // Sent: topic1 Message 0
    // Sent: topic1 Message 1
    // Sent: topic1 Message 2
    // Sent: topic1 Message 3
    // Sent: topic1 Message 4
    // Sent: topic1 Message 5
    // Sent: topic1 Message 6
    // Sent: topic1 Message 7
    // Sent: topic1 Message 8
    // Sent: topic1 Message 9
}

pub fn test_all()
{
    // rep_req::run();
    pub_sub::run();
}