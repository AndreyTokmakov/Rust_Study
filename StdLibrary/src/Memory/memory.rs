
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


pub fn test_all()
{
    // string_test();
    // string_test_use_after_move();
    // clone_string();
    drop_string();
}

