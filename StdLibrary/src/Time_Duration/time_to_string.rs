use std::thread::sleep;
use std::time::Duration;
use chrono::Local;

const DATE_FORMAT: &'static str = "[%Y-%m-%d %H:%M:%S.%3f]";


fn format_to_string()
{
    println!("{} Sleeping", Local::now().format(DATE_FORMAT));
    sleep(Duration::new(0, 500 * 1000 * 1000));
    println!("{} Awake", Local::now().format(DATE_FORMAT));
}

pub fn test_all()
{
    format_to_string();
}