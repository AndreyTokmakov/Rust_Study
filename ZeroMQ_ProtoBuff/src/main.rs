#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod server_1;
mod multiple_handlers;

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

    static PORT: i32 = 52525;

    pub fn init_tracing()
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

        socket.bind(format!("tcp://127.0.0.1:{}", PORT).as_str()).unwrap();
        info!("Server started");

        loop {
            let msg = socket.recv_string(0).unwrap().unwrap();
            info!(message = %msg, "Received request");

            socket.send("OK", 0).unwrap();
            info!("Response sent");
        }
    }
}

pub fn main()
{
    // person_demo::test_item();
    // person_demo::test_message();

    // Initialize a global subscriber for logging
    /*tracing_subscriber::fmt()
        .with_max_level(Level::INFO) // set log level
        .init();*/

    // zmq_tests::init_tracing();
    // zmq_tests::run_server();

    // server_1::run_server();
    multiple_handlers::run_server();
}