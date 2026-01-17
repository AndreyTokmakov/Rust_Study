

#[path = "examples/examples.rs"] pub mod examples;
#[path = "examples/finding_element_in_collection.rs"] pub mod finding_element_in_collection;
#[path = "examples/parsing_strings_to_numbers.rs"] pub mod parsing_strings_to_numbers;
#[path = "examples/simple_cache.rs"] pub mod simple_cache;


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

fn map() // Преобразует значение, если оно есть.
{
    let x = Some(5);
    let y: Option<i32> = x.map(|v| v * 2);
    println!("{:?}", y); // Some(10)
}

fn half(x: i32) -> Option<i32> {
    if x % 2 == 0 {
        Some(x / 2)
    } else {
        None
    }
}


fn and_then() // Преобразует значение, если оно есть.
{
    let result = Some(8).and_then(half).and_then(half);
    println!("{:?}", result); // Some(2)

    let fail = Some(7).and_then(half);
    println!("{:?}", fail); // None
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

//noinspection ALL
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

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn check_if_value_exists()
{
    match divide(10, 2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero"),
    }

    match divide(10, 0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero"),
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

fn unwrap()
{
    let optStr: Option<&str> = Some("qwerty");
    println!("{:?}", optStr.unwrap());

    // unwrap panics with a generic message
    let optStr: Option<&str> = None;
    println!("{:?}", optStr.unwrap());
}

fn unwrap_or()
{
    let some_value: Option<&str> = Some("Something");
    let none_value: Option<&str> = None;

    println!("{}", some_value.unwrap_or("Default")); // Output: Something
    println!("{}", none_value.unwrap_or("Default")); // Output: Default
}

fn unwrap_or_else()
{
    // Returns the contained Some value or computes it from a closure.
    let optStr: Option<&str> = Some("qwerty");
    println!("{:?}", optStr.unwrap_or_else(|| "Default_Value"));

    let optStr: Option<&str> = None;
    println!("{:?}", optStr.unwrap_or_else(|| "Default_Value"));

    let default = || {
        println!("Computing default value...");
        "Computed default"
    };

    let none_value: Option<&str> = None;
    println!("{}", none_value.unwrap_or_else(default));

    // "qwerty"
    // "Default_Value"
    // Computing default value...
    // Computed default
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


fn expect()
{
    let some_value: Option<&str>  = Some("Hello, world!");
    println!("{}", some_value.expect("This won't panic")); // Output: Hello, world!

    // The line below would panic with our custom message
    let none_value: Option<&str> = None;
    println!("{}", none_value.expect("We expected a string but got None!"));
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
    // INFO : Inserts value into the option *** IF IT'S == None *** then returns a mutable reference to the contained value.
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


// https://www.compilenrun.com/docs/language/rust/rust-enums/rust-option-enum
pub fn test_all()
{ 
    // create_optional();
    // create_and_test();
    // take();
    // map();
    // and_then();
    // take_if();
    // question_mark_operator();

    // pattern_matching();  // if let
    // check_if_value_exists();

    // unwrap();
    // unwrap_or();
    // unwrap_or_else();
    // unwrap_unchecked();
    // or();
    // ok_or();
    // ok_or_else();
    // expect();

    // is_none();
    // is_some();

    // insert();
    // get_or_insert();
    // filter();

    // examples::test_all();
    // finding_element_in_collection::test_all();
    // parsing_strings_to_numbers::test_all();
    simple_cache::test_all();
}

// Method	            Description	                                                Example

// is_some()	        Returns true if the option is a Some value	                Some(42).is_some() → true
// is_none()	        Returns true if the option is a None value	                None::<i32>.is_none() → true
// unwrap()	            Returns the contained value or panics if None	            Some("hi").unwrap() → "hi"
// expect(msg)	        Like unwrap() but with a custom panic message	            Some(4).expect("Failed") → 4
// unwrap_or(default)	Returns the contained value or a default	                None.unwrap_or(10) → 10
// unwrap_or_else(f)	Returns the contained value or computes it from a closure	None.unwrap_or_else(
// map(f)	            Applies a function to the contained value (if any)	                    Some(2).map(
// map_or(default, f)	Applies a function to the contained value or returns a default	        None.map_or(42,
// ok_or(err)	        Transforms Option<T> into Result<T, E>	                                Some(v).ok_or(err) → Ok(v)
// and(optb)	        Returns None if self is None, otherwise returns optb	                Some(2).and(Some("hi")) → Some("hi")
// or(optb)	            Returns self if it is Some, otherwise returns optb	                    None.or(Some(100)) → Some(100)
// and_then(f)	        Returns None if self is None, otherwise calls f and returns the result	Chain operations that might fail
// filter(predicate)	Returns None if self is None or the predicate returns false	            Keep only values matching a condition