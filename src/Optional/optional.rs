
fn create_optional()
{
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_char);
    println!("{:?}", absent_number);
}

fn create_and_test()
{
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
}

fn create_and_test_2()
{
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_none(), true);
}

fn is_none()
{
    let a: Option<u32> = Some(2);
    println!("a is_none: {}", a.is_none());

    let b: Option<u32> = None;
    println!("b is_none: {}", b.is_none());
}

fn is_some()
{
    let a: Option<u32> = Some(2);
    println!("a is_some: {}", a.is_some());

    let b: Option<u32> = None;
    println!("b is_some: {}", b.is_some());
}

fn ok_or_else()
{
    let x = Some("foo");
    assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or_else(|| 0), Err(0));
}


pub fn test_all()
{
    // create_optional();
    // create_and_test();
    // create_and_test_2();


    // is_none();
    // is_some();
    ok_or_else();
}