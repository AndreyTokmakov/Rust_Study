
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Person
{
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Address
{
    street: String,
    city: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Model
{
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "v")]
    pub value: i32,
    #[serde(rename = "d")]
    pub description: String,
}


fn person_from_string() -> Result<()>
{
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the same function as the one
    // that produced serde_json::Value above, but now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    println!("{:?}", p);
    Ok(())
}

fn print_an_address() -> Result<()>
{
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
    Ok(())
}

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


fn parse_json_rename_fields()-> Result<()>
{
    let json_str = r#"{"n": "John Doe", "v": 43, "d": "Something"}"#;
    
    let model: Model = serde_json::from_str(json_str)?;
    println!("{:?}", model);

    let json_str_2: String = serde_json::to_string(&model)?;
    println!("{:?}", json_str_2);
    
    Ok(())
    
    // Model { name: "John Doe", value: 43, description: "Something" }
    // "{\"n\":\"John Doe\",\"v\":43,\"d\":\"Something\"}"
}


pub fn test_all()
{
    // parse_json_string();
    // parse_json_string_2();

    parse_json_rename_fields().expect("TODO: panic message");
    
    // person_from_string().expect("Can't print Person");
    // print_an_address().expect("Can't print Address");
}