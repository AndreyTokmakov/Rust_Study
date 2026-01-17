use std::num::ParseIntError;

mod parsing_strings_to_numbers
{

}

pub fn test_all()
{
    let valid_number: Result<i32, ParseIntError> = "42".parse::<i32>();
    let invalid_number: Result<i32, ParseIntError>  = "not a number".parse::<i32>();

    println!("Valid: {:?}", valid_number);
    println!("Invalid: {:?}", invalid_number);

    // Using Option methods to handle the results
    let doubled: Option<i32> = valid_number.ok().map(|x| x * 2);
    println!("Doubled: {:?}", doubled);

    // Providing a default for invalid input
    let value: i32 = invalid_number.unwrap_or(0);
    println!("With default: {}", value);

    // Valid: Ok(42)
    // Invalid: Err(ParseIntError { kind: InvalidDigit })
    // Doubled: Some(84)
    // With default: 0
}