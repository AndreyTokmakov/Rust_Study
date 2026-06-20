
mod basics
{
    use clap::builder::Str;

    pub fn create()
    {
        let data = "One";
        let s1: String = data.to_string();
        let s2: String = "Two".to_string();
        let s3: String = "Three".to_owned();

        println!("{} {} {}", s1, s2, s3);
    }

    pub fn append_to_string()
    {
        let mut s: String = String::from("foo ");
        println!("{}", s);

        s.push_str("bar");
        println!("{}", s);
    }

    pub fn append_with_string()
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

    pub fn concatenate_2()
    {
        let name: &str  = "Alice";
        let message: String  = format!("Hello, {}!", name);
        println!("{}", message);
    }

    pub fn concatenate_push_back()
    {
        let mut text: String = String::from("Hello");
        text.push_str(" world");

        println!("{}", text); // -> Hello world
    }

    pub fn concat(a: &str, b: &str) -> String
    {
        format!("{}{}", a, b)
    }

    pub fn concatenate_3()
    {
        let result: String  = concat("Hello ", "World");
        println!("{}", result);
    }

    pub fn format_string()
    {
        let s1: String = String::from("tic");
        let s2: String = String::from("tac");
        let s3: String = String::from("toe");

        let s: String = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }

    pub fn iterate_string_0()
    {
        let str = String::from("hello");
        for c in str.chars() {
            println!("{}", c);
        }
    }

    pub fn iterate_string_enumerate()
    {
        let str = String::from("hello");
        for (i, c) in str.chars().enumerate() {
            println!("str[{}] = {}", i, c);
        }
    }

    pub fn iterate_string_bytes()
    {
        let str: String = String::from("привет");
        for b in str.bytes() {
            println!("{}", b);
        }
    }

    pub fn split_string()
    {
        let text = "hello world wonderful world1";
        for word in text.split_whitespace() {
            println!("{}", word);
        }
    }
}

mod examples
{
    fn first_word(s: &str) -> &str
    {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    pub fn returns_first_word()
    {
        let my_string: String = String::from("Hello world");

        let word: &str = first_word(&my_string);
        println!("First word: {}", word); // ✅ Output: First word: Hello

        let my_string_literal: &str = "Hello world";
        let word: &str = first_word(my_string_literal);
        println!("First word: {}", word); // ✅ Output: First word: Hello
    }
}


pub fn test_all()
{
    // basics::create();

    // basics::append_to_string();
    // basics::append_with_string();
    // basics::concatenate_1();
    // basics::concatenate_2();
    // basics::concatenate_3();
    // basics::concatenate_push_back();

    // basics::format_string();
    // basics::iterate_string_0();
    // basics::iterate_string_enumerate();
    // basics::iterate_string_bytes();
    // basics::split_string();

    examples::returns_first_word();
}
