#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod files;
mod paths;
mod folders_directories;
mod asynch_io;
mod monitoring;

pub fn main()
{
    asynch_io::test_all();
    // files::test_all();
    // paths::test_all();
    // folders_directories::test_all();
    // monitoring::test_all();
}
