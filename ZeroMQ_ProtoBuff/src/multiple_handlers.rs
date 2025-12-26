
mod multiple_handlers
{
    use zmq::{Context, Socket};
    use tracing::{info, error, instrument};
    use std::thread;
    use tracing_subscriber::FmtSubscriber;
    use tracing::Level;


    pub struct EchoHandler;
    pub struct UppercaseHandler;
    pub struct ReverseHandler;

    pub trait RequestHandler: Send + 'static {
        fn handle(&self, msg: &str) -> String;
    }

    impl RequestHandler for EchoHandler
    {
        fn handle(&self, msg: &str) -> String {
            format!("echo: {}", msg)
        }
    }

    impl RequestHandler for UppercaseHandler
    {
        fn handle(&self, msg: &str) -> String
        {
            msg.to_uppercase()
        }
    }

    impl RequestHandler for ReverseHandler
    {
        fn handle(&self, msg: &str) -> String
        {
            msg.chars().rev().collect()
        }
    }

    pub struct ZmqServer
    {
        socket: Socket,
        handler: Box<dyn RequestHandler + Send>,
    }

    impl ZmqServer
    {
        pub fn new(endpoint: &str, handler: Box<dyn RequestHandler + Send>) -> Self
        {
            let ctx: Context = Context::new();
            let socket: Socket = ctx.socket(zmq::REP).unwrap();
            socket.bind(endpoint).unwrap();

            println!("Server bound to {}", endpoint);
            Self { socket, handler }
        }

        pub fn run(&self)
        {
            loop  {
                let msg: String = self.socket.recv_string(0).unwrap().unwrap();
                let reply: String = self.handler.handle(&msg);
                self.socket.send(&reply, 0).unwrap();
            }
        }
    }

    fn init_tracing()
    {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_target(false)
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    pub fn main()
    {
        init_tracing();

        let servers: Vec<(String, Box<dyn RequestHandler + Send>)> = vec![
            ("tcp://127.0.0.1:5555".to_string(), Box::new(EchoHandler)),
            ("tcp://127.0.0.1:5556".to_string(), Box::new(UppercaseHandler)),
            ("tcp://127.0.0.1:5557".to_string(), Box::new(ReverseHandler)),
        ];

        let mut handles = Vec::new();
        for (endpoint, handler) in servers {
            let handle = thread::spawn(move || {
                let server = ZmqServer::new(&endpoint, handler);
                server.run();
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}

pub fn run_server()
{
    multiple_handlers::main();
}