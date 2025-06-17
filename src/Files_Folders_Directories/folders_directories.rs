
use std::io;
use std::fs::{ self, DirEntry };
use std::path::Path;

extern crate walkdir;
use walkdir::WalkDir;


fn list_files_in_directory(path: &Path)
{
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{:?}", entry.file_name());
                // println!("{}", file.unwrap().path().display());
            }
        }
    }
    else {
        println!("Directory {:?} not found", path);
    }
}

fn list_files_in_current_directory()
{
    list_files_in_directory(Path::new("."));
}


fn is_path_exist(path: &str)
{
    let exist: bool = Path::new(path).exists();
    if exist {
        println!("Path {:?} exist ", path);
    }
    else {
        println!("Path {:?} not found", path);
    }
}

fn list_files_only_recursive(path: &str)
{
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            println!("{}", file.path().display());
        }
    }
}


pub fn test_all()
{
    // list_files_in_current_directory();
    // list_files_in_directory(Path::new("/home/andtokm/DiskS/Temp"));

    // is_path_exist("/home/andtokm/DiskS/Temp");
    // is_path_exist("/home/andtokm/DiskS/Temp11");

    list_files_only_recursive("/home/andtokm/DiskS/Temp");
}