
#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]


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

pub fn main()
{
    // person_demo::test_item();
    person_demo::test_message();
}