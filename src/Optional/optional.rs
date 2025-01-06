
mod examples;

fn create_optional()
{
    let number_1: Option<i32> = Some(5);
    let number_2: Option<i32> = Option::from(6);
    let some_char: Option<char>  = Some('e');
    let absent_number: Option<i32> = None;

    println!("{:?}", number_1);
    println!("{:?}", number_2);

    println!("{:?}", some_char);
    println!("{:?}", absent_number);
}

fn create_and_test()
{
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    assert_eq!(x.is_none(), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);
    assert_eq!(x.is_none(), true);
}

fn sum_opts(a: Option<i32>, b: Option<i32>) -> Option<i32>
{
    // Some(a + b)    //   --->   error[E0369]: cannot add `Option<i32>` to `Option<i32>`
    Some(a? + b?)
}

fn question_mark_operator()
{
    for (a, b) in [(Some(1), Some(2)), (Some(1), None)].iter() {
        println!("{:?} + {:?} = {:?}", a.clone(), b.clone(), sum_opts(a.clone(), b.clone()));
    }
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
    let x: Option<&str> = Some("foo");
    assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or_else(|| 0), Err(0));
}


pub fn test_all()
{
    // create_optional();
    // create_and_test();
    take();
    // question_mark_operator();


    // is_none();
    // is_some();
    // ok_or_else();

    // examples::test_all();
}