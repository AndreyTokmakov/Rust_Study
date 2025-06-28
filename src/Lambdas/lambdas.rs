

fn simple_lambda_return()
{
    let get_value : fn() -> i32 = || 100500;
    println!("{}", get_value());
}


fn simple_lambda_with_return()
{
    fn function(i: i32) -> i32 {
        i + 1
    }
    let result: i32 = function(100);

    println!("{}", result);
}

fn check_optional_with_lambda()
{
    let check_msg: fn(Option<&str>) = | opt_str: Option<&str> |
    {
        // Take a reference to the contained string
        if let Some(m) = &opt_str {
            println!("{}", *m);
        }

        // Remove the contained string, destroying the Option
        let unwrapped_msg: &str = opt_str.unwrap_or("default message");
        println!("{}\n", unwrapped_msg);
    };

    let msg: Option<&str> = Some("Hello World!");
    check_msg(msg);
}


fn call_lambda_inplace()
{
    fn ten_times<F>(f: F) where F: Fn(i32) {
        for index in 0..3 {
            f(index);
        }
    }

    ten_times(|j| println!("hello, {}", j));

    let word = "konnichiwa".to_owned();
    ten_times(move |j| println!("{}, {}", word, j));
}


fn examples__is_even()
{
    let is_even: fn(i32) -> bool = |n: i32| 0 == n & 1;

    for v in 0.. 10 {
        println!("Is {} even = {}", v, is_even(v));
    }
}


pub fn test_all()
{
    // simple_lambda_return();
    // simple_lambda_with_return();
    // check_optional_with_lambda();
    // call_lambda_inplace();

    examples__is_even();
}