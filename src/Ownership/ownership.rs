

fn pass_by_reference()
{
    let s: String = String::from("hello");
    not_takes_ownership(&s);
    let len: usize = calculate_length(&s);
    println!("String '{}', Length: {}", s, len);
}

fn not_takes_ownership(str: &String)
{
    println!("{}", str);
}

fn calculate_length(s: &String) -> usize
{
    return s.len();
}

fn test1()
{
    let s = String::from("hello");  // s comes into scope

    // s's value moves into the function... and so is no longer valid here
    takes_ownership(s);

    // value borrowed here after move
    // println!("{}", s);   // ERROR

    // x comes into scope
    let x = 5;

    // x would move into the function,but i32 is Copy, so it's okay to still use x afterward
    makes_copy(x);
}

fn takes_ownership(some_string: String)
{
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32)
{
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



pub fn test_all()
{
    // test1();
    pass_by_reference();
}
