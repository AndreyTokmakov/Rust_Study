
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
        let tasks = vec![
            thread::spawn(|| { publisher(); }),
            thread::spawn(|| { subscriber(); })
        ];

        for task in tasks {
            task.join().unwrap();
        }
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

mod push_pull
{
    use std::thread;
    use std::thread::sleep;
    use zmq::{Context, Socket};
    use std::time::Duration;

    static PORT: i32 = 5557;

    // PUSH
    fn producer()
    {
        let context: Context = zmq::Context::new();
        let producer: Socket = context.socket(zmq::PUSH).unwrap();
        producer.bind(format!("tcp://*:{PORT}").as_str()).unwrap();

        println!("Producer ready on tcp://*:{}", PORT);

        for i in 0..10 {
            let task: String = format!("Task #{}", i);
            println!("Sending {}", task);
            producer.send(&task, 0).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    }

    // PULL
    fn worker()
    {
        let context: Context = zmq::Context::new();
        let worker: Socket = context.socket(zmq::PULL).unwrap();
        worker.connect(format!("tcp://localhost:{PORT}").as_str()).unwrap();

        loop {
            let msg: String = worker.recv_string(0).unwrap().unwrap();
            println!("Worker received: {}", msg);
        }
    }

    pub fn run()
    {
        let tasks = vec![
            thread::spawn(|| { producer(); }),
            thread::spawn(|| { worker(); })
        ];

        // Wait for all threads to finish
        for task in tasks {
            task.join().unwrap();
        }
    }
}

mod multipart
{
    use std::thread;
    use zmq::{Context, Socket};

    pub fn run()
    {
        let context: Context = zmq::Context::new();
        let sender: Socket = context.socket(zmq::PUSH).unwrap();
        let receiver: Socket = context.socket(zmq::PULL).unwrap();

        sender.bind("inproc://test").unwrap();
        receiver.connect("inproc://test").unwrap();

        // Send multipart message
        sender.send("Header", zmq::SNDMORE).unwrap();
        sender.send("Body", 0).unwrap();

        // Receive multipart
        let header: String = receiver.recv_string(0).unwrap().unwrap();
        let body: String = receiver.recv_string(0).unwrap().unwrap();
        println!("Multipart received: [{}] [{}]", header, body);
    }

    // Multipart received: [Header] [Body]
}

mod router_dealer_1
{
    use zmq::{Context, Socket};
    use std::{thread, time::Duration};
    use chrono::{DateTime, Local};
    use chrono::format::{DelayedFormat, StrftimeItems};

    fn log(level: &str, msg: &str)
    {
        let now: DateTime<Local> = Local::now();
        let timestamp: DelayedFormat <StrftimeItems> = now.format("%Y-%m-%d %H:%M:%S.%3f");
        println!("[{:<5}] [{}] {}", level, timestamp, msg);
    }

    macro_rules! info {
        ($($arg:tt)*) => {
                log("INFO", &format!($($arg)*));
            };
    }

    fn broker()
    {
        let ctx: Context = zmq::Context::new();
        let frontend: Socket = ctx.socket(zmq::ROUTER).unwrap();
        frontend.bind("tcp://*:5559").unwrap();

        let backend: Socket = ctx.socket(zmq::DEALER).unwrap();
        backend.bind("tcp://*:5560").unwrap();

        info!("[Broker] Async-like broker ROUTER<->DEALER started");
        thread::spawn(move || {
            let mut items = [
                frontend.as_poll_item(zmq::POLLIN),
                backend.as_poll_item(zmq::POLLIN),
            ];

            loop {
                zmq::poll(&mut items, 100).unwrap();
                // info!("Got message");

                if items[0].is_readable() {
                    let msg = frontend.recv_multipart(0).unwrap();
                    backend.send_multipart(msg, 0).unwrap();
                }

                if items[1].is_readable() {
                    let msg = backend.recv_multipart(0).unwrap();
                    frontend.send_multipart(msg, 0).unwrap();
                }
            }
        });

        loop {
            // Broker runs forever (or add shutdown logic)
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    fn client()
    {
        info!("[Client] is starting....");

        let ctx: Context = zmq::Context::new();
        let client: Socket = ctx.socket(zmq::REQ).unwrap();
        client.connect("tcp://localhost:5559").unwrap();

        info!("[Client] connected to: {}", "tcp://localhost:5559");
        for i in 0..5 {
            let msg: String = format!("[Client] msg {}", i);
            client.send(msg.as_bytes(), 0).unwrap();


            let reply: String = client.recv_string(0).unwrap().unwrap();
            info!("[Client] Got reply: {}", reply);
            std::thread::sleep(Duration::from_millis(300));
        }
    }

    fn worker()
    {
        info!("[Worker] is starting....");

        let ctx: Context = zmq::Context::new();
        let worker: Socket = ctx.socket(zmq::REP).unwrap();
        worker.connect("tcp://localhost:5560").unwrap();

        info!("[Worker] connected to: {}", "tcp://localhost:5560");
        loop {
            let msg = worker.recv_string(0).unwrap().unwrap();
            info!("[Worker] Received: {}", msg);
            let reply: String = format!("Processed by worker: {}", msg);
            worker.send(reply.as_str(), 0).unwrap();
        }
    }
    pub fn run()
    {
        let tasks = vec![
            thread::spawn(|| { broker(); }),
            thread::spawn(|| { worker(); }),
            thread::spawn(|| { client(); })
        ];

        // Wait for all threads to finish
        for task in tasks {
            task.join().unwrap();
        }
    }
}

mod router_dealer_2
{
    use zmq::{Context, Socket};
    use std::{thread, time::Duration};
    use chrono::{DateTime, Local};
    use chrono::format::{DelayedFormat, StrftimeItems};

    fn log(level: &str, msg: &str)
    {
        let now: DateTime<Local> = Local::now();
        let timestamp: DelayedFormat <StrftimeItems> = now.format("%Y-%m-%d %H:%M:%S.%3f");
        println!("[{:<5}] [{}] {}", level, timestamp, msg);
    }

    macro_rules! info {
        ($($arg:tt)*) => {
                log("INFO", &format!($($arg)*));
            };
    }

    fn broker()
    {
        let context: Context = zmq::Context::new();

        // ROUTER accepts client connections
        let frontend: Socket = context.socket(zmq::ROUTER).unwrap();
        frontend.bind("tcp://*:5559").unwrap();

        // DEALER connects to workers
        let backend: Socket = context.socket(zmq::DEALER).unwrap();
        backend.bind("tcp://*:5560").unwrap();

        info!("Broker started on 5559(front) <-> 5560(back)");

        // Use zmq::proxy to forward between sockets
        zmq::proxy(&frontend, &backend).unwrap();
    }

    fn client()
    {
        let context: Context = zmq::Context::new();
        let socket: Socket = context.socket(zmq::REQ).unwrap();
        socket.connect("tcp://localhost:5559").unwrap();

        for i in 0..5 {
            let msg: String = format!("Hello from client {}", i);
            socket.send(&msg, 0).unwrap();

            let reply: String = socket.recv_string(0).unwrap().unwrap();
            info!("Client got reply: {}", reply);

            thread::sleep(Duration::from_millis(500));
        }
    }

    fn worker()
    {
        let context: Context = zmq::Context::new();
        let worker: Socket = context.socket(zmq::REP).unwrap();
        worker.connect("tcp://localhost:5560").unwrap();

        loop {
            let msg: String = worker.recv_string(0).unwrap().unwrap();
            info!("Worker received: {}", msg);
            worker.send(format!("Processed: {}", msg).as_str(), 0).unwrap();
        }
    }

    pub fn run()
    {
        let tasks = vec![
            thread::spawn(|| { broker(); }),
            thread::spawn(|| { worker(); }),
            thread::spawn(|| { client(); })
        ];

        // Wait for all threads to finish
        for task in tasks {
            task.join().unwrap();
        }
    }
}


pub fn test_all()
{
    // rep_req::run();
    pub_sub::run();
    // push_pull::run();
    // multipart::run();

    // router_dealer_1::run();
    // router_dealer_2::run();
}