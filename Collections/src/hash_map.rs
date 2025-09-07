use std::collections::HashMap;

pub fn create()
{
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
}

pub fn find()
{
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 11);
    scores.insert(String::from("Yellow"), 123);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);
}

fn iterate()
{
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn add_or_insert()
{
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(20);
    scores.entry(String::from("Blue")).or_insert(30);

    println!("{:?}", scores);
}

fn update_values()
{
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(20);
    scores.entry(String::from("Blue")).insert_entry(30);

    println!("{:?}", &scores);
}

fn count_words_in_the_text()
{
    let text = "hello world wonderful world";

    let mut counter = HashMap::new();
    for word in text.split_whitespace() {
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counter);
}

fn check_contains_key()
{
    let mut table: HashMap<String, i32> = HashMap::new();

    table.insert(String::from("One"), 1);
    table.insert(String::from("Two"), 2);
    table.insert(String::from("Three"), 3);

    for (key, value) in &table {
        println!("{}: {}", key, value);
    }

    let key: String = String::from("One");
    if table.contains_key("One") {
        println!("{} exists", &key);
    }
}

fn check_contains_key_list()
{
    let mut table: HashMap<String, i32> = HashMap::new();

    table.insert(String::from("One"), 1);
    table.insert(String::from("Two"), 2);
    table.insert(String::from("Three"), 3);

    // Look up the values associated with some keys.
    let keys_to_find = ["One", "Two", "Five"];
    for &key in &keys_to_find {
        match table.get(key) {
            Some(value) => println!("{key} ==> {value}"),
            None => println!("{key} not found.")
        }
    }
}

fn delete_key()
{
    let mut table: HashMap<String, i32> = HashMap::new();

    table.insert(String::from("One"), 1);
    table.insert(String::from("Two"), 2);
    table.insert(String::from("Three"), 3);

    for (key, value) in &table {
        println!("{}: {}", key, value);
    }

    let key: String = String::from("One");
    if table.contains_key("One") {
        println!("{} exists", &key);
    }

    table.remove(&key);


    if table.contains_key("One") {
        println!("{} exists", &key);
    }
    else {
        println!("{} not found", &key);
    }
}

pub fn test_all()
{
    // create();
    // find();
    // iterate();
    // add_or_insert();
    // update_values();
    // count_words_in_the_text();

    // check_contains_key();
    check_contains_key_list();

    // delete_key();
}