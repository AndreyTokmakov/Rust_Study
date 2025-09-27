#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod serialise_deserialize_objects;
mod read_write_from_file;
mod handle_errors;
mod nested_structures;

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

fn parse_array()
{
    let data = r#"
        [
            { "id": 1, "name": "Alice" },
            { "id": 2, "name": "Bob" }
        ]
    "#;

    let v: Value = serde_json::from_str(data).unwrap();

    // Access as array
    if let Some(arr) = v.as_array() {
        for item in arr {
            println!("id = {}, name = {}", item["id"], item["name"]);
        }
    }

    // id = 1, name = "Alice"
    // id = 2, name = "Bob"
}


fn modify_json_in_place()
{
    let mut v: Value = json!({
        "id": 1,
        "name": "Alice",
        "tags": ["rust", "json"]
    });


    v["active"] = Value::Bool(true);                       // Add new field
    v["name"] = Value::String("Alice Smith".to_string());  // Modify existing

    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}


pub fn main()
{
    // parse_json_string();
    // parse_json_string_2();
    // parse_array();
    // modify_json_in_place();

    // serialise_deserialize_objects::test_all();
    // read_write_from_file::test_all();
    // handle_errors::test_all();
    nested_structures::test_all();
}