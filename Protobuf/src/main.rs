

mod person_demo
{
    pub mod person {
        include!(concat!(env!("OUT_DIR"), "/example.rs"));
    }

    use person::Person;
    use prost::Message;

    pub fn test_all()
    {

        let person = Person {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
        };

        // Serialize to bytes
        let encoded = prost::Message::encode_to_vec(&person);
        println!("Encoded bytes: {:?}", encoded);

        // Deserialize from bytes
        let decoded = Person::decode(&*encoded).unwrap();
        println!("Decoded Person = {:?}", decoded);
    }
}

mod store_demo
{
    pub mod store
    {
        pub mod items {
            include!(concat!(env!("OUT_DIR"), "/store.items.rs"));
        }
    }

    use prost::Message;
    use store::items::Shirt;
    use store::items::shirt::Size;

    pub fn test_all()
    {
        let mut shirt: Shirt = Shirt::default();
        shirt.color = "blue".to_string();
        shirt.size = Size::Medium.into(); // Convert enum to i32 for protobuf

        assert_eq!(shirt.color, "blue");
        assert_eq!(shirt.size, Size::Medium as i32);

        let bytes: Vec<u8> = prost::Message::encode_to_vec(&shirt);
        println!("Encoded bytes: {:?}", bytes);

        let shirt_decoded: Shirt = Shirt::decode(&*bytes).unwrap();
        println!("Shirt Person = {:?}", shirt_decoded);
    }
}

pub fn main()
{
    // person_demo::test_all();
    store_demo::test_all();
}