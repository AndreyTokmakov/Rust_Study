

use tracing_subscriber::FmtSubscriber;
use tracing::Level;

mod server_1
{
    use zmq::{Context, Socket};
    use tracing::{info, error, instrument, Level};
    use tracing_subscriber::FmtSubscriber;


    pub struct EchoHandler;

    pub trait RequestHandler  {
        fn handle(&self, msg: &str) -> String;
    }

    impl RequestHandler for EchoHandler {
        fn handle(&self, msg: &str) -> String
        {
            format!("echo: {}", msg)
        }
    }

    pub struct ZmqServer<H>
        where H: RequestHandler,
    {
        socket: Socket,
        handler: H,
        running: bool,
    }

    impl<H> ZmqServer<H>
        where H: RequestHandler,
    {
        pub fn new(endpoint: &str, handler: H) -> Self
        {
            let ctx = Context::new();
            let socket = ctx.socket(zmq::REP).unwrap();

            socket.bind(endpoint).unwrap();

            Self {
                socket,
                handler,
                running: false,
            }
        }

        #[instrument(skip(self))]
        pub fn run(&mut self)
        {
            self.running = true;
            info!("ZeroMQ REP server started");

            while self.running {
                match self.socket.recv_string(0) {
                    Ok(Ok(msg)) => {
                        info!(request = %msg, "Received request");

                        let reply = self.handler.handle(&msg);
                        self.socket.send(&reply, 0).unwrap();

                        info!(response = %reply, "Sent response");
                    }

                    Ok(Err(_)) => {
                        error!("Invalid UTF-8 message");
                        self.socket.send("ERR", 0).unwrap();
                    }

                    Err(e) => {
                        error!(error = %e, "ZeroMQ recv failed");
                        break;
                    }
                }
            }
        }

        pub fn stop(&mut self)
        {
            self.running = false;
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

    pub fn run()
    {
        init_tracing();

        let handler = EchoHandler;
        let mut server = ZmqServer::new(
            "tcp://127.0.0.1:52525",
            handler,
        );

        server.run();
    }
}

pub fn run_server()
{
    server_1::run();
}
