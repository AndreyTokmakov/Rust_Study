
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn open_file()
{
    // let greeting_file_result = File::open("hello.txt");
    let greeting_file_result = File::open("/home/andtokm/tmp/folder_for_testing/trace.log");

    println!("{:?}", greeting_file_result);
}

fn read_file_1()
{
    let file: File  = File::open("/home/andtokm/Temp/headers.txt").unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();

    loop {
        let len: usize = reader.read_line(&mut line).unwrap();
        if 0 == len {
            break;
        }
        println!("{}", line);
        line.clear();
        // line.truncate(0);
    }
}

fn read_file_2()
{
    let file: File  = File::open("/home/andtokm/Temp/headers.txt").unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);


    for line_ in reader.lines() {
        let line: String = line_.unwrap();
        println!("{}", line);
    }
}

pub fn test_all()
{
    // open_file();
    read_file_1();
    // read_file_2();
}
