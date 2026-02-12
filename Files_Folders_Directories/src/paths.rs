use std::env;
use std::path::{Path, PathBuf};

fn inspect(path: &PathBuf)
{
    println!("\npath: {:?}", path);
    println!("  file_name: {:?}", path.file_name());
    println!("  extension: {:?}", path.extension());
    println!("  exist    : {:?}", path.exists());
    println!("  is_dir   : {:?}", path.is_dir());
    println!("  is_file  : {:?}", path.is_file());
}

fn get_current_dir()
{
    let curr_dir: PathBuf = env::current_dir().unwrap();
    println!("Current directory: {:?}", curr_dir);
}

fn make_path_buf()
{
    let path: PathBuf = PathBuf::from("/tmp/hello.txt");
    inspect(&path);
}

fn make_path()
{
    let path: &Path = Path::new("/tmp/hello.txt");
}

fn path_attributes_tests()
{
    inspect(&env::current_dir().unwrap().join("../resources/input.txt"));
    inspect(&env::current_dir().unwrap().join("../resources"));
}


pub fn test_all()
{
    // get_current_dir();
    // make_path_buf();
    // make_path();
    path_attributes_tests();
}