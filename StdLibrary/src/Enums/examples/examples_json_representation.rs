
mod json_representation
{
    use std::collections::HashMap;
    use clap::builder::Str;

    enum JsonValue {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(HashMap<String, JsonValue>),
    }

    fn describe_json(value: &JsonValue, indent: usize) {
        let indent_str: String = " ".repeat(indent);

        match value {
            JsonValue::Null => println!("{}Null value", indent_str),
            JsonValue::Boolean(b) => println!("{}Boolean: {}", indent_str, b),
            JsonValue::Number(n) => println!("{}Number: {}", indent_str, n),
            JsonValue::String(s) => println!("{}String: \"{}\"", indent_str, s),
            JsonValue::Array(items) => {
                println!("{}Array with {} items:", indent_str, items.len());
                for (i, item) in items.iter().enumerate() {
                    print!("{}- Item {}: ", indent_str, i);
                    describe_json(item, indent + 2);
                }
            }
            JsonValue::Object(map) => {
                println!("{}Object with {} properties:", indent_str, map.len());
                for (key, value) in map {
                    print!("{}- {}: ", indent_str, key);
                    describe_json(value, indent + 2);
                }
            }
        }
    }

    pub fn demo()
    {
        // Create a complex JSON structure
        let mut user: HashMap<String, JsonValue> = HashMap::new();
        user.insert("name".to_string(), JsonValue::String("John Doe".to_string()));
        user.insert("age".to_string(), JsonValue::Number(30.0));
        user.insert("is_active".to_string(), JsonValue::Boolean(true));

        let mut address: HashMap<String, JsonValue> = HashMap::new();
        address.insert("street".to_string(), JsonValue::String("123 Main St".to_string()));
        address.insert("city".to_string(), JsonValue::String("Springfield".to_string()));

        user.insert("address".to_string(), JsonValue::Object(address));

        let hobbies: Vec<JsonValue> = vec![
            JsonValue::String("coding".to_string()),
            JsonValue::String("reading".to_string()),
            JsonValue::String("hiking".to_string()),
        ];

        user.insert("hobbies".to_string(), JsonValue::Array(hobbies));

        let json = JsonValue::Object(user);

        describe_json(&json, 0);
    }
}

pub fn test_all()
{
    json_representation::demo();

    // Object with 5 properties:
    // - name:   String: "John Doe"
    // - hobbies:   Array with 3 items:
    //   - Item 0:     String: "coding"
    //   - Item 1:     String: "reading"
    //   - Item 2:     String: "hiking"
    // - is_active:   Boolean: true
    // - age:   Number: 30
    // - address:   Object with 2 properties:
    //   - city:     String: "Springfield"
    //   - street:     String: "123 Main St"
}