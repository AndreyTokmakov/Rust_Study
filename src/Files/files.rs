
use std::fs::File;

pub fn test_all()
{
    // let greeting_file_result = File::open("hello.txt");
    let greeting_file_result = File::open("/home/andtokm/tmp/folder_for_testing/trace.log");

    println!("{:?}", greeting_file_result);
}
