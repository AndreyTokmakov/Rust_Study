#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod serialise_deserialize_objects;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};

fn parse_json_string()
{
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);
    println!("{}", john.to_string());
}

fn parse_json_string_2()
{
    let full_name: &str = "John Doe";
    let age_last_year: i32 = 42;
    let phone: &str = "+44 1234567";

    // The type of `john` is `serde_json::Value`
    let john: Value = json!({
        "name": full_name,
        "age": age_last_year + 1,
        "phones": [
            format!("+44 {}", phone)
        ]
    });

    println!("{}", john);
}



pub fn main()
{
    // parse_json_string();
    // parse_json_string_2();

    serialise_deserialize_objects::test_all();

}