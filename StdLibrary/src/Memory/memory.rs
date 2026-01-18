
fn string_test()
{
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);  // ERROR!!
    println!("{}", s2);
}

fn string_test_use_after_move()
{
    let s1 = String::from("hello");
    {
        let s2 = s1;
        println!("{}", s2);
    }
    // let s3: String = s1;  // ERROR
}

fn clone_string()
{
    let s1 = String::from("Test");
    {
        let s2 = s1.clone();
        println!("{}", s1);
    }
    println!("{}", s1);
}

fn drop_string()
{
    let s1 = String::from("hello");

    println!("{}", s1);

    drop(s1);

    // println!("{}", s1);  // ---> Error
}

mod replace
{
    use std::mem;

    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
    }

    // Analog of C++ std::exchange
    pub fn simple_test()
    {
        let mut x: i32 = 5;
        let old: i32 = mem::replace(&mut x, 10);

        println!("Old value: {}, New value: {}", old, x); // 5
    }

    pub fn simple_test_optional()
    {
        let mut value = Some(42);
        let old: Option<i32> = mem::replace(&mut value, None);

        println!("Old: {:?}, Current: {:?}", old, value);
    }

    pub fn exchange_part_of_structure()
    {
        let mut user = User {
            name: "Alice".to_string(),
            age: 30,
        };

        // Safely replace name without partial move
        let old_name: String = mem::replace(&mut user.name, "Unknown".to_string());

        println!("{:?}", old_name);
        println!("{:?}", user);
    }
}


pub fn test_all()
{
    // string_test();
    // string_test_use_after_move();
    // clone_string();
    // drop_string();

    // replace::simple_test();
    // replace::simple_test_optional();
    // replace::exchange_part_of_structure();
}

