
mod files;
mod paths;
mod folders_directories;
mod asynch_io;

pub fn test_all()
{
    asynch_io::test_all();
    // files::test_all();
    // paths::test_all();
    // folders_directories::test_all();
}