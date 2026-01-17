

fn divide(numerator: i32, denominator: i32) -> Option<i32>
{
    if denominator == 0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_number(numerator: i32, denominator: i32)
{
    let result: Option<i32>  = divide(numerator, denominator);
    match result {
        Some(x) => println!("Result: {x}"),
        None    => println!("Cannot divide by 0"),
    }
}


fn divide_test()
{
    for (a, b) in [(4,2), (5,3), (1, 0), (0, 1),].iter() {
        divide_number(a.clone(), b.clone());
    }
}


fn check_optional(optional: Option<Box<i32>>)
{
    match optional {
        Some(p) => println!("has value {p}"),
        None => println!("has no value"),
    }
}

fn check_boxed()
{
    let optional: Option<Box<i32>> = None;
    check_optional(optional);

    let optional: Option<Box<i32>> = Some(Box::new(9000));
    check_optional(optional);
}


pub fn test_all()
{
    // divide_test();
    check_boxed();
}