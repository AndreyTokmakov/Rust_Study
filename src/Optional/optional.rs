
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

fn take() // Takes the value out of the option, leaving a None in its place.
{
    let mut orig: Option<i32> = Some(2);
    let res: Option<i32> = orig.take();

    assert_eq!(orig, None);
    assert_eq!(res, Some(2));

    println!("{:?}", orig);
    println!("{:?}", res);
}

fn take_if()
{
    println!("{}", "-".repeat(120).as_str());
    {
        let mut orig: Option<i32> = Some(123);
        // FALSE - the 'orig' will NOT be transferred to 'res' (since 'orig' != 122)
        let res: Option<i32> = orig.take_if(|v| *v == 122);

        println!("orig = {:?}", orig);
        println!("res =  {:?}", res);
    }

    println!("{}", "-".repeat(120).as_str());
    {
        let mut orig: Option<i32> = Some(123);
        // OK - the 'orig' will be transferred to 'res'
        let res: Option<i32> = orig.take_if(|v| *v == 123);

        println!("orig = {:?}", orig);
        println!("res =  {:?}", res);
    }

    println!("{}", "-".repeat(120).as_str());
    {
        let mut orig: Option<i32> = Some(123);

        // No TAKE happens BUT orig will be changed to orig = oric - 23
        let res: Option<i32> = orig.take_if(|v| if *v == 123 {
            *v -= 23;
            false
        } else {
            false
        });

        println!("orig = {:?}", orig);
        println!("res =  {:?}", res);
    }
    println!("{}", "-".repeat(120).as_str());
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

fn pattern_matching()
{
    let check_message = |opt_str: Option<&str>|
    {
        // Take a reference to the contained string
        if let Some(m) = &opt_str {
            println!("{}", *m);
        }

        // Remove the contained string, destroying the Option
        let unwrapped_msg: &str = opt_str.unwrap_or("default message");
        println!("{}\n", unwrapped_msg);
    };

    let msg: Option<&str> = Some("howdy");
    check_message(msg);

    let msg: Option<&str> = None;
    check_message(msg);
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

fn unwrap()
{
    let optStr: Option<&str> = Some("qwerty");
    println!("{:?}", optStr.unwrap());

    // unwrap panics with a generic message
    let optStr: Option<&str> = None;
    println!("{:?}", optStr.unwrap());
}

fn unwrap_or_else()
{
    // Returns the contained Some value or computes it from a closure.
    let optStr: Option<&str> = Some("qwerty");
    println!("{:?}", optStr.unwrap_or_else(|| "Default_Value")); // "qwerty"

    let optStr: Option<&str> = None;
    println!("{:?}", optStr.unwrap_or_else(|| "Default_Value")); // "Default_Value"
}

fn unwrap_unchecked()
{
    // Returns the contained Some value, consuming the self value, without checking that the value is not None.
    // Calling this method on None is undefined behavior.

    let optStr: Option<&str> = Some("qwerty");
    println!("{:?}", unsafe { optStr.unwrap_unchecked()});

    let optStr: Option<&str> = None;
    println!("{:?}", unsafe { optStr.unwrap_unchecked()});
}

fn or()
{
    // Returns the option if it contains a value, otherwise returns .
    let optStr: Option<&str> = Some("qwerty");
    let defOptStr: Option<&str>= Some("Default_Value");
    println!("{:?}", optStr.or(defOptStr));

    let optStr: Option<&str> = None;
    let defOptStr: Option<&str>= Some("Default_Value");
    println!("{:?}", optStr.or(defOptStr));

    // Output: Some("qwerty")
    //         Some("Default_Value")
}

fn ok_or()
{
    // Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
    let optStr: Option<&str> = Some("qwerty");
    println!("{:?}",optStr.ok_or(-1));

    let optStr: Option<&str> = None;
    println!("{:?}",optStr.ok_or(-1));

    // Output: Ok("qwerty")
    //         Err(-1)
}

fn ok_or_else()
{
    // Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err()).

    let optStr: Option<&str> = Some("qwerty");
    println!("{:?}",optStr.ok_or_else(|| 123));

    let optStr: Option<&str> = None;
    println!("{:?}",optStr.ok_or_else(|| 123));

    // Output: Ok("foo")
    //         Err(123)
}

fn insert()
{
    // Inserts value into the option, then returns a mutable reference to it.
    // If the option already contains a value, the old value is dropped.

    let mut optStr: Option<&str> = Some("qwerty");
    println!("Original value: {:}", optStr.unwrap());

    let oldValue: &mut &str = optStr.insert("New_Value");
    println!("Updated  value: {:}", oldValue);

    // Output: Original value: qwerty
    //         Updated  value: New_Value
}

fn get_or_insert()
{
    /// Inserts value into the option *** IF IT'S == None *** then returns a mutable reference to the contained value.
    let mut optStr: Option<&str> = Some("qwerty");
    println!("Original value: {:}", optStr.unwrap());

    let oldValue: &mut &str = optStr.get_or_insert("New_Value");
    println!("Updated  value: {:}", oldValue);

    // Output: Original value: qwerty
    //         Updated  value: qwerty
}

fn filter()
{
    let is_even: fn(&i32) -> bool = |n: &i32| 0 == n & 1;

    println!("{:?}", None.filter(is_even));
    println!("{:?}", Some(3).filter(is_even));
    println!("{:?}", Some(4).filter(is_even));
}

pub fn test_all()
{
    // create_optional();
    // create_and_test();
    // take();
    // take_if();
    question_mark_operator();
    // pattern_matching();

    // unwrap();
    // unwrap_or();
    // unwrap_or_else();
    // unwrap_unchecked();
    // or();
    // ok_or();
    // ok_or_else();

    // is_none();
    // is_some();

    // insert();
    // get_or_insert();
    // filter();

    // examples::test_all();
}