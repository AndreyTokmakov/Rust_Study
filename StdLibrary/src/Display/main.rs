
mod demo_one
{
    use std::fmt;
    use std::fmt::{Display};
    
    #[derive(Debug,PartialEq)]
    enum FileState {
        Open,
        Closed
    }

    #[derive(Debug)]
    struct File
    {
        name: String,
        content: String,
        state: FileState
    }

    impl File {
        fn new(name: &str, data: &str) -> File {
            File {
                name: String::from(name),
                content: String::from(data),
                state: FileState::Closed,
            }
        }
    }

    impl Display for FileState 
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            match *self {
                FileState::Open => write! (f, "OPEN"),
                FileState::Closed => write! (f, "CLOSED"),
            }
        }
    }

    impl Display for File 
    {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            write!(format, "File [ Name = {}, State = {} ]",self.name, self.state)
        }
    }
    
    pub fn custom_Display_implementation()
    {
        let file_1: File = File::new("f6.txt", "123232323");
        println! ("{:?}", file_1);
        println! ("{}", file_1);
    }
}


pub fn test_all()
{
    demo_one::custom_Display_implementation();
}