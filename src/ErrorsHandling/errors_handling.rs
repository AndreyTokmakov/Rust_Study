use std::{error};
use std::fs::File;
use std::io::{self, Error, Read};
use std::io::ErrorKind;
use std::io::ErrorKind::InvalidData;
use std::num::ParseIntError;

fn string2Double(number_str: &str) -> Result<i32, ParseIntError>
{
    match number_str.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}


fn panic_test()
{
    panic!("crash and burn");
}

fn out_borders_index_error()
{
    let  v = vec![1, 2, 3];
    let x = v[99];
}

fn handle_file_open_error()
{
    // let greeting_file_result = File::open("hello.txt");
    let greeting_file_result = File::open("/home/andtokm/tmp/f2older_for_testing/trace.log");

    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("File opened successfully");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn handle_file_open_error_2()
{
    let greeting_file = File::open("hello.txt").unwrap();
}

fn handle_file_open_error_3()
{
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn experiments()
{
    // let parseResult: Result<i32, ParseIntError> = string2Double("10");
    // println!("{:?}", parseResult);
}

//------------------------------------------------------------------------------

fn getPositiveOnly(val: i32) -> Result<i32, String> {
    if val >= 0 {
        Ok(val)
    }
    else {
        Err(String::from("Negative value"))
    }
}

fn simple_custom_error() {

    println!("{:?}", getPositiveOnly(11));
    println!("{:?}", getPositiveOnly(-11));
    println!("{:?}", getPositiveOnly(0));
}

//------------------------------------------------------------------------------

fn propagateError_oldsStyle(number_str: &str) -> Result<i32, ParseIntError> {
    let parseResult = string2Double(number_str);

    let mut value = return match parseResult {
        Ok(value) => Ok(value),
        Err(e) => Err(e),
    };
}

fn get_positive_only(res: Result<i32, ParseIntError>) -> Result<i32, String> {
    let val = res.unwrap();
    if val >= 0 {
        Ok(val)
    } else {
        Err(String::from("Negative value"))
    }
}

fn propagate_error_test() {
    let res = propagateError_oldsStyle("-12");
    println!("{:?}", res);

    let res2 = get_positive_only(res);
    println!("{:?}", res2);
}

//------------------------------------------------------------------------------


fn read_username_from_file() -> Result<String, io::Error>
{
    let username_file_result: io::Result<File> = File::open("hello1.txt");

    let mut username_file: File = match username_file_result {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file");
            return Err(e)
        },
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => {
            println!("Failed to read string");
            Err(e)
        },
    }
}

fn propagate_error_test_manual_check() -> Result<String, Error> {
    read_username_from_file()
}

//------------------------------------------------------------------------------

fn propagate_error_test_new_style() -> Result<String, io::Error>
{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


pub fn test_all()
{
    // panic_test();
    // out_borders_index_error();

    // handle_file_open_error();
    // handle_file_open_error_2();
    // handle_file_open_error_3();

    // simple_custom_error();

    // propagate_error_test();
    let error: Result<String, io::Error> = propagate_error_test_manual_check();

    // experiments();
}