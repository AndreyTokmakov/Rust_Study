use std::env;
use std::path::PathBuf;

mod read_file
{
    use std::env;
    use std::fs::File;
    use std::io::{self, Read};
    use std::path::PathBuf;

    fn read_file_contents(path: &str) -> Result<String, io::Error>
    {
        let mut file: File = match File::open(path) {
            Ok(file) => file,
            Err(error) => return Err(error),
        };

        let mut contents: String = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(error) => Err(error),
        }
    }

    pub fn demo(filename: &str)
    {
        match read_file_contents(filename) {
            Ok(contents) => {
                println!("File contents:{}", contents);
            }
            Err(error) => {
                println!("Error reading file: {}", error);
            }
        }
    }
}

pub fn test_all()
{
    let file_path: PathBuf = env::current_dir().unwrap().join("resources/test_files/file1.txt");

    read_file::demo(file_path.to_str().unwrap());
    read_file::demo("some_missing_file.txt");
}