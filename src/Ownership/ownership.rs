
fn not_takes_ownership(str: &String)
{
    println!("{}", str);
}

fn calculate_length(s: &String) -> usize
{
    return s.len();
}

fn pass_by_const_reference()
{
    let s: String = String::from("hello");
    not_takes_ownership(&s);
    let len: usize = calculate_length(&s);
    println!("String '{}', Length: {}", s, len);
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


fn test1()
{
    let s = String::from("hello");  // s comes into scope

    // s's value moves into the function... and so is no longer valid here
    takes_ownership(s);

    /** value borrowed here after move **/
    // println!("{}", s);   /// ----> ERROR

    // x comes into scope
    let x = 5;

    // x would move into the function,but i32 is Copy, so it's okay to still use x afterward
    makes_copy(x);
}


fn immutable_borrowing()
{
    let s: String = String::from("Hello, Rust!");
    let str_ref_1: &String = &s;  /// Immutable borrow
    let str_ref_2: &String = &s;  /// Another immutable borrow

    println!("{} and {}", str_ref_1, str_ref_2); // Works fine
}

fn mutable_borrowing()
{
    let mut str_orig: String = String::from("Hello");
    let str_ref: &mut String = &mut str_orig;

    /** Will not compile | second mutable borrow occurs here **/
    // let str_ref2: &mut String = &mut str_orig;

    str_ref.push_str(" world");
    println!("{}", str_orig);
}

fn use_after_move()
{
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); // <---- WILL NOT COMPILE | ^^^^ value borrowed here after move
    println!("{s2}, world!");
}


pub fn test_all()
{

    use_after_move();

    // test1();
    // pass_by_const_reference();

    // immutable_borrowing();
    // mutable_borrowing();
}
