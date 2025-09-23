
pub fn test_all()
{
    // create();
    // append_to_string();
    // append_with_string();
    // concatenate_1();
    // concatenate_2();
    concatenate_3();


    // format_string();

    // iterate_string_0();
    // iterate_string_enumerate();
    // iterate_string_bytes();

    // split_string();
}

fn create()
{
    let data = "One";
    let s1: String = data.to_string();
    let s2: String = "Two".to_string();
    let s3: String = "Three".to_owned();

    println!("{} {} {}", s1, s2, s3);
}


fn append_to_string() 
{
    let mut s: String = String::from("foo ");
    println!("{}", s);

    s.push_str("bar");
    println!("{}", s);
}

fn append_with_string()
{
    let mut s1: String = String::from("foo");
    let mut s2: String = String::from("bar");
    s1.push_str(&s2);

    println!("s1 is {}\ns2 is {}", s1, s2);

    s2.push_str("_bar");

    println!("s1 is {}\ns2 is {}", s1, s2);
}

fn concatenate_1()
{
    let s1: String = String::from("Hello");
    let s2: &str = " world!";

    let concatenated: String = s1 + s2; // s1 is moved here
    println!("{}", concatenated);
}

fn concatenate_2()
{
    let name: &str  = "Alice";
    let message: String  = format!("Hello, {}!", name);
    println!("{}", message);
}

fn concat(a: &str, b: &str) -> String
{
    format!("{}{}", a, b)
}

fn concatenate_3()
{
    let result: String  = concat("Hello ", "World");
    println!("{}", result);
}

fn format_string()
{
    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");

    let s: String = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
}


fn iterate_string_0() {
    let str = String::from("hello");
    for c in str.chars() {
        println!("{}", c);
    }
}

fn iterate_string_enumerate()  {
    let str = String::from("hello");

    for (i, c) in str.chars().enumerate() {
        println!("str[{}] = {}", i, c);
    }
}

fn iterate_string_bytes()  {
    let str = String::from("привет");

    for b in str.bytes() {
        println!("{}", b);
    }
}

fn split_string() {
    let text = "hello world wonderful world1";

    for word in text.split_whitespace() {
        println!("{}", word);
    }

}