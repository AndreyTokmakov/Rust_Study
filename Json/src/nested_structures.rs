
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    address: Address,
}



pub fn test_all()
{
    let json_str = r#"
        {
            "id": 1,
            "name": "Alice",
            "address": {
                "city": "Paris",
                "zip": "75000"
            }
        }
    "#;

    let user: User = serde_json::from_str(json_str).unwrap();
    let back: String = serde_json::to_string_pretty(&user).unwrap();

    println!("{:?}", user);
    println!("{}", back);
}