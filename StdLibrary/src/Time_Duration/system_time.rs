
use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn test()
{
    let now: SystemTime = SystemTime::now();
    sleep(Duration::new(2, 0));
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
}

pub fn test_all()
{
    // test();
    
    let now: SystemTime = SystemTime::now();
    println!("{:?}", now);
}