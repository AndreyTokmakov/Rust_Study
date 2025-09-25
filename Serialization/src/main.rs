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
struct Point {
    x: i32,
    y: i32,
}

fn serialize_struct()
{
    let p = Point { x: 10, y: -5 };

    // Serialize to Vec<u8>
    let encoded: Vec<u8> = bincode::serialize(&p).unwrap();

    // Deserialize back
    let decoded: Point = bincode::deserialize(&encoded).unwrap();

    println!("Original: {:?}", p);
    println!("Encoded : {:?}", encoded);
    println!("Decoded : {:?}", decoded);
}

fn main()
{
    serialize_struct();

}
