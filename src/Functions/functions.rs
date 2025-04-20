use std::ops::Add;

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn func_with_macro() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn six() -> i32
{
    return 6;
}

fn plus_one(x: i32) -> i32
{
    x + 1
}

fn init_variable_with_func() {
    let a = five();
    println!("a: {}", a);

    let b = six();
    println!("b: {}", b);

    let c = plus_one(6);
    println!("c: {}", c);
}

fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len(); // len() returns the length of a String

    // return (s, length);  // Same
    (s, length)
}

fn get_tuple()
{
    let s: String = String::from("Test");
    let (s2, len) = calculate_length(s);
    println!("The length of '{}' is {}.", s2, len);
}

fn foo(s: &mut String)
{
    s.push_str("_Updated!");
}

fn update_value_in_function()
{
    let mut str: String = String::from("12345");
    println!("{}", str);
    foo(&mut str);
    println!("{}", str);
}


pub fn test_all()
{
    // another_function(5);
    // print_labeled_measurement(5, 'h');
    // func_with_macro();
    // init_variable_with_func();
    get_tuple();
    // update_value_in_function();
}