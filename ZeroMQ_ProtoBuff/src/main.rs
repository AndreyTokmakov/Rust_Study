#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

use tracing::Level;

mod person_demo
{
    pub mod store { include!("../src/generated/store.rs");  }
    pub mod protocol {  include!("../src/generated/protocol.rs"); }

    use store::Item;
    // use ;
    use prost::Message;

    pub fn test_item()
    {
        let item = Item {
            name: "Alice".to_string(),
            id: 30,
            description: "alice@example.com".to_string(),
            description2: "alice@example.com".to_string(),
        };

        let encoded: Vec<u8> = prost::Message::encode_to_vec(&item);
        println!("Encoded bytes: {:?}", encoded);

        let decoded = Item::decode(&*encoded).unwrap();
        println!("Decoded Person = {:?}", decoded);
    }

    pub fn test_message()
    {
        let msg = protocol::Message {
            len: 12,
            data: "Payload".to_string(),
        };

        let encoded: Vec<u8> = prost::Message::encode_to_vec(&msg);
        println!("Encoded bytes: {:?}", encoded);

        let decoded: protocol::Message = protocol::Message::decode(&*encoded).unwrap();
        println!("Decoded Person = {:?}", decoded);
    }
}


pub mod zmq_tests
{
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;
    use zmq::Context;
    use tracing::{info, instrument};

    fn init_tracing()
    {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_target(false)
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    #[instrument(name = "rep_server")]
    pub fn run_server()
    {
        let ctx = Context::new();
        let socket = ctx.socket(zmq::REP).unwrap();

        socket.bind("tcp://127.0.0.1:5555").unwrap();
        info!("Server started");

        loop {
            let msg = socket.recv_string(0).unwrap().unwrap();
            info!(message = %msg, "Received request");

            socket.send("OK", 0).unwrap();
            info!("Response sent");
        }
    }

    #[instrument(name = "req_client", fields(client_id = id))]
    pub fn run_client(id: u64)
    {
        let ctx = Context::new();
        let socket = ctx.socket(zmq::REQ).unwrap();

        socket.connect("tcp://127.0.0.1:5555").unwrap();

        let payload = format!("hello from {}", id);
        info!(payload = %payload, "Sending request");

        socket.send(&payload, 0).unwrap();

        let reply = socket.recv_string(0).unwrap().unwrap();
        info!(reply = %reply, "Received reply");
    }

}

pub fn main()
{
    // person_demo::test_item();
    // person_demo::test_message();

    // Initialize a global subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO) // set log level
        .init();

    zmq_tests::run_server();
}