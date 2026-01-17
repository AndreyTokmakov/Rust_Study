

mod utils
{
    pub fn divide(a: i32, b: i32) -> Result<i32, String>
    {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    pub fn square(x: i32) -> Result<i32, String> {
        Ok(x * x)
    }

    pub fn parse_number(str_number: &str) -> Result<i32, String>
    {
        match str_number.parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!("Failed to parse '{}'", str_number)),
        }
    }
}


mod methods
{
    use crate::result::utils;

    pub fn unwrap()
    {
        let res: Result<i32, &str> = Ok(42);
        println!("{}", res.unwrap()); // 42

        let err: Result<i32, &str> = Err("fail");
        // println!("{}", err.unwrap()); // PANIC!
    }

    pub fn unwrap_or()
    {
        let res: Result<i32, &str> = Ok(42);
        println!("{}", res.unwrap_or(0)); // 42

        let err: Result<i32, &str> = Err("fail");
        println!("{}", err.unwrap_or(0)); // 0
    }

    pub fn map()
    {
        let x: Result<i32, &str> = Ok(5);
        let y: Result<i32, &str> = x.map(|v| v * 2);
        println!("{:?}", y); // Ok(10)
    }

    pub fn map_err()
    {
        let x: Result<i32, &str> = Err("oops");
        let y: Result<i32, String> = x.map_err(|e| format!("Error: {}", e));

        println!("{:?}", y); // --->  Err("Error: oops")
    }

    pub fn and_then()
    {
        let r = Ok(3).and_then(utils::square).and_then(utils::square);
        println!("{:?}", r); // Ok(81)
    }

    pub fn and_then_2()
    {
        // and_then: Chain operations that might fail
        let result: Result<i32, String> = utils::divide(12, 2).and_then(|x| utils::divide(x, 2));
        println!("{:?}", result); // Output: Ok(2)
    }

    pub fn or_else()
    {
        let result: Result<i32, String> = utils::divide(10, 0).or_else(|_| {
            println!("<or_else> branch called");
            utils::divide(8, 2)
        });
        println!("{:?}", result);

        // <or_else> branch called
        // Ok(4)
    }

    fn parse_and_divide(a: &str, b: &str) -> Result<i32, String>
    {
        let x: i32 = a.parse().map_err(|_| "Invalid number".to_string())?;
        let y: i32 = b.parse().map_err(|_| "Invalid number".to_string())?;

        if y == 0 {
            return Err("Division by zero".to_string());
        }
        Ok(x / y)
    }

    pub fn operator__question_mark() // Главный «сахар» в Rust. Позволяет писать короткий код вместо кучи match.
    {
        println!("{:?}", parse_and_divide("10", "2"));
        println!("{:?}", parse_and_divide("10", "0"));
        println!("{:?}", parse_and_divide("10", "x"));

        // Ok(5)
        // Err("Division by zero")
        // Err("Invalid number")
    }
}


mod usages_and_examples
{
    use crate::result::utils;
    pub fn parse_number_test()
    {
        match utils::parse_number("42") {
            Ok(n) => println!("Parsed: {}", n),
            Err(e) => println!("Error: {}", e),
        }

        match utils::parse_number("abc") {
            Ok(n) => println!("Parsed: {}", n),
            Err(e) => println!("Error: {}", e),
        }

        // Using unwrap_or
        let n = utils::parse_number("123").unwrap_or(0);
        println!("n = {}", n);
    }

    pub fn divide_test()
    {
        match utils::divide(10, 2) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}",  err),
        }

        match utils::divide(10, 0) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}",  err),
        }
    }

    pub fn divide_test_using_if_let()
    {
        if let Ok(value) = utils::divide(10, 2) {
            println!("The result is: {}", value);
        }
        if let Err(error) =  utils::divide(5, 0) {
            println!("An error occurred: {}", error);
        }

        // The result is: 5
        // An error occurred: Division by zero
    }
}


mod use_cases
{
    use std::io::{self, Read};
    use std::{env, fs};
    use std::path::PathBuf;
    use std::fs::File;
    use std::num::ParseIntError;

    fn read_file_to_string(path: &str) -> Result<String, io::Error>
    {
        let mut file: File = File::open(path)?; // if error → return Err
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn read_file_1()
    {
        match read_file_to_string(env::current_dir().expect("REASON").join("resources/Cargo.toml").to_str().unwrap()) {
            Ok(text) => println!("File content:\n{}", text),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn read_file() -> Result<(), std::io::Error>
    {
        let file_path: String = env::current_dir()?.join("resources/Cargo.toml").to_str().unwrap().to_string();
        let content: String = fs::read_to_string(file_path)?;
        println!("File content:\n{}", content);
        Ok(())
    }

    pub fn parsing_string_to_number()
    {
        let parser = |str_num: String| {
            let fail: Result<i32, ParseIntError> = str_num.parse();
            match fail {
                Ok(n) => println!("Number: {}", n),
                Err(e) => println!("Error parsing: {}", e),
            }
        };

        parser("123".to_string());
        parser("abc".to_string());

        // Number: 123
        // Error parsing: invalid digit found in string
    }

    fn square_root(x: f64) -> Result<f64, String> {
        if x < 0.0 {
            Err("Negative numbers have no real square root".to_string())
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn calc_square_root()
    {
        println!("{:?}", square_root(9.0));   // Ok(3.0)
        println!("{:?}", square_root(-1.0));  // Err("Negative numbers...")
    }
}

mod customisation_tests
{
    #[derive(Debug)]
    enum MyError {
        NotFound,
        InvalidInput,
    }

    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        Overflow,
        Underflow,
    }

    fn divide_with_custom_error(a: i32, b: i32) -> Result<i32, MathError>
    {
        if b == 0 {
            Err(MathError::DivisionByZero)
        } else if a == i32::MAX && b == -1 {
            Err(MathError::Overflow)
        } else {
            Ok(a / b)
        }
    }

    fn get_item(id: i32) -> Result<String, MyError>
    {
        if id < 0 {
            Err(MyError::InvalidInput)
        } else if id == 0 {
            Err(MyError::NotFound)
        } else {
            Ok(format!("Item {}", id))
        }
    }

    pub fn custom_error_type()
    {
        match get_item(0) {
            Ok(item) => println!("Found: {}", item),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    pub fn devive_error_type()
    {
        let result: Result<i32, MathError> = divide_with_custom_error(10, 0);
        match result {
            Ok(value) => println!("Result: {}", value),
            Err(MathError::DivisionByZero) => println!("Cannot divide by zero!"),
            Err(MathError::Overflow) => println!("Operation would cause overflow!"),
            Err(MathError::Underflow) => println!("Operation would cause underflow!"),
        }
    }
}

/**

Result определён так:

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

Result<T, E> — это главный способ обработки ошибок в Rust.
Если Option — это «есть/нет значение», то Result — это «успех или ошибка».

Ok(T) — успешный результат с данными типа T.
Err(E) — ошибка с информацией типа E.

Методы:
✅ unwrap и unwrap_or
✅ map
✅ map_err
✅ and_then
✅ ? оператор

**/

// INFO: https://www.compilenrun.com/docs/language/rust/rust-enums/rust-result-enum

pub fn test_all()
{
    // methods::map();
    // methods::map_err();
    // methods::unwrap();
    // methods::unwrap_or();
    // methods::and_then();
    // methods::and_then_2();
    // methods::or_else();
    // methods::operator__question_mark();

    // use_cases::read_file().expect("TODO: panic message");
    // use_cases::read_file_1();
    // use_cases::parsing_string_to_number();
    // use_cases::calc_square_root();

    // usages_and_examples::parse_number_test();
    // usages_and_examples::divide_test();
    // usages_and_examples::divide_test_using_if_let();

    // customisation_tests::custom_error_type();
    customisation_tests::devive_error_type();
}