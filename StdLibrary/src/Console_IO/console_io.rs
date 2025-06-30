
use std::env;

pub fn test_all()
{
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
