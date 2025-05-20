
use std::time::{Duration};

pub fn test_all()
{
    let five_seconds = Duration::from_secs(5);

    assert_eq!(five_seconds, Duration::from_millis(5_000));
    assert_eq!(five_seconds, Duration::from_micros(5_000_000));
    assert_eq!(five_seconds, Duration::from_nanos(5_000_000_000));
    
    println!("{:?}", five_seconds);
}