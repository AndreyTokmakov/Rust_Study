#![allow(unused_variables)]

mod file_tests
{
    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    impl File 
    {
        fn new(name: &str) -> File {
            File {
                name: String::from(name),
                data: Vec::new()
            }
        }
    }

    fn open(file: &mut File) -> bool {
        true
    }

    fn close(file: &mut File) -> bool {
        true
    }

    #[allow(dead_code)]
    fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
        unimplemented!()
    }
    
    
    pub fn demo()
    {
        let mut file1: File = File::new( "fl.txt");

        open(&mut file1);
        close(&mut file1);
    }
}



mod return_values
{
    // Note: Function will run forever --> return nothing
    fn forever() -> ! {
        loop { }
    }
    
    // Note: Function return () implicitly 
    fn return_tuple()  {
    }


    fn dev(a: i32, b: i32) -> Result<String, String> {
        if (0 == b) {
            Err(String::from("Error: b == 0"))
        }
        else {   
            Ok((a/b).to_string())
        }
    }
    
    pub fn return_result_or_err()  
    {
        let res = dev(10, 0);
        println!("{}", res.unwrap());
    }
}


use chrono::Local;

const DATE_FORMAT: &'static str = "[%Y-%m-%d %H:%M:%S.%3f] ";

#[macro_export]
macro_rules! info {
    () => {
        println!("{}", Local::now().format(DATE_FORMAT));
    };
    ($($arg:tt)*) => {{
        println!($($arg)*)
    }};
}



pub fn test_all()
{
    // file_tests::demo();
    // return_values::return_result_or_err();

    info!();
    info!("{}", 123);
    // dbg!("{}", 123);
}