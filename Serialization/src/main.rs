#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

use bincode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person
{
    name: String,
    age: u8,
    email: String,
}


fn serialize_struct()
{
    let person = Person {
        name: String::from("Murat"),
        age: 25,
        email: String::from("murat@example.com"),
    };

    let serialized: Vec<u8> = bincode::serialize(&person).unwrap();
    println!("Serialized: {:?}", serialized);

    let deserialized: Person = bincode::deserialize(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

fn main()
{
    serialize_struct();

}
