use std::f32::consts::E;
use clap::builder::TypedValueParser;

fn get_from_optional()
{
    let maybe_number: Option<i32> = Some(5);
    let number: i32 = maybe_number.unwrap(); // вернёт 5
    println!("number = {}", number);
}

fn get_from_result()
{
    let result: Result<i32, &str> = Ok(10);
    let number: i32 = result.unwrap(); // вернёт 10
    println!("number = {}", number);
}

fn get_from_result_with_expect()
{
    let result: Result<i32, &str> = Err("Ops");
    let number = result.expect("Bang!!!");
    println!("number = {}", number);
    
    // thread 'main' panicked at src/ErrorsHandling/unwrap.rs:21:25:
    // Bang!!!: "Ops"
}


/// Метод .unwrap() в Rust используется для извлечения значения из типов Option<T> или Result<T, E>,
/// и он применяется тогда, когда ты уверен, что там точно есть значение и не будет ошибки.
pub fn test_all()
{
    get_from_optional();
    get_from_result();
    get_from_result_with_expect();
}