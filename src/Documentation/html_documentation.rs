
//! Simulating files one step at а time.

/// Represents а "file",
/// which probaЬly lives on а file system.
#[derive(Debug)]
struct File
{
    name: String,
    data: Vec<u8>,
}

impl File
{
    /// New files are assumed to Ье empty, but а name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new()
        }
    }

    /// Returns the file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }


    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

// INFO: Call 'rustdoc html_documentation.rs'
//   cd doc/

pub fn test_all()
{
    let file: File = File::new("fl.txt");
    let name: String = file.name();
    let length: usize = file.len();
    
    println! ("{:?}", file);
    println! ("{} is {} bytes long", name, length);
}