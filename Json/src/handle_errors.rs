use serde_json;

fn test_simple()
{
    let bad_json = "{ id: 1, name: Alice }"; // некорректный JSON

    match serde_json::from_str::<serde_json::Value>(bad_json) {
        Ok(v) => println!("Parsed: {}", v),
        Err(e) => println!("Error parsing JSON: {}", e),
    }

    // Error parsing JSON: key must be a string at line 1 column 3
}


pub fn test_all()
{
    test_simple();
}