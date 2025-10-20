use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn open_file()
{
    let file_path: PathBuf = env::current_dir().unwrap().join("resources/input.txt");
    let greeting_file_result = File::open(file_path);

    println!("{:?}", greeting_file_result);
}

fn read_file_0()
{
    let file_path: PathBuf = env::current_dir().unwrap().join("resources/Cargo.toml");
    let mut file: File = File::open(file_path).unwrap(); // if error → return Err
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("TODO: panic message");
    println!("File contents:\n{}", contents);
}

fn read_file_1()
{
    let file_path: PathBuf = env::current_dir().unwrap().join("resources/input.txt");
    let file: File  = File::open(file_path).unwrap();
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

fn read_file__buffer_reader()
{
    let file_path: PathBuf = env::current_dir().unwrap().join("resources/input.txt");
    let file: File  = File::open(file_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    
    for line_ in reader.lines() {
        let line: String = line_.unwrap();
        println!("{}", line);
    }
}

fn read_file__buffer_reader_2()
{
    let file_path: PathBuf = env::current_dir().unwrap().join("resources/test_files/in_out.txt");
    let file: File = File::open(file_path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents).expect("*** FAILED ***");
    
    println!("{}", contents);
}

fn write_file()
{
    let file_path: PathBuf = env::current_dir().unwrap().join("resources/test_files/in_out.txt");
    let data: &str = "Some data!\n";
    
    let mut file: File = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open(file_path)
        .expect("Unable to open file");
    
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

pub fn test_all()
{
    // open_file();
    read_file_0();
    // read_file_1();
    // read_file__buffer_reader();
    // read_file__buffer_reader_2();

    // write_file();
}
