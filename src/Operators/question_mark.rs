
mod demo_one 
{
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        InvalidNumber,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn return_result(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::InvalidNumber)
        } else {
            Ok(x)
        }
    }

    fn call_div(x: f64, y: f64) -> MathResult {
        // if `div` "fails", then `DivisionByZero` will be `return`ed
        let ratio: f64 = div(x, y)?;
        return_result(ratio)
    }

    pub fn op(x: f64, y: f64)
    {
        match call_div(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                MathError::DivisionByZero => "division by zero",
                MathError::InvalidNumber => "Invalid Number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
    
    pub fn test()
    {
        op(1.0, 2.0);
        println!("==========================================================================");
        op(2.0, 1.0);
        println!("==========================================================================");
        op(2.0, 0.0);
    }
}

mod read_file_example
{
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, Read};


    fn read_username_from_file_old(file_name: String) -> Result<String, io::Error> 
    {
        let username_file_result = File::open(file_name);

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username: String = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file_new(file_name: String) -> Result<String, io::Error> 
    {
        let mut username: String = String::new();
        File::open(file_name)?.read_to_string(&mut username)?;
        Ok(username)
    }
    
    
    pub fn test()
    {
        read_username_from_file_old(String::from("/home/andtokm/DiskS/ProjectsUbuntu/Rust_Study/resources/user_data.txt"))
            .expect("Failed to get data");
        read_username_from_file_new(String::from("/home/andtokm/DiskS/ProjectsUbuntu/Rust_Study/resources/user_data.txt"))
            .expect("Failed to get data");
    }
}

pub fn test_all()
{
    // demo_one::test();
    
    read_file_example::test();
}