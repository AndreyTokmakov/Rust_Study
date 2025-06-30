use std::fs;
use walkdir::WalkDir;

fn print_directory_files() {
    for file in fs::read_dir("/home/andtokm/tmp/folder_for_testing").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}

fn print_directory_files_WalkDir() {
    for entry in WalkDir::new("/home/andtokm/tmp/folder_for_testing").into_iter() {
        println!("{:?}", entry);
    }
}

pub fn test_all()
{
    // print_directory_files();
    print_directory_files_WalkDir();
}