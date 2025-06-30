use std::thread::sleep;
use std::time::{Duration, Instant};



fn create_Duration()
{
    let one_sec: Duration = Duration::ZERO;
    assert_eq!(one_sec, Duration::from_secs(0));
    
    let five_seconds: Duration = Duration::from_secs(5);

    assert_eq!(five_seconds, Duration::from_millis(5_000));
    assert_eq!(five_seconds, Duration::from_micros(5_000_000));
    assert_eq!(five_seconds, Duration::from_nanos(5_000_000_000));

    println!("{:?}", five_seconds);

    let ten_seconds: Duration = Duration::from_secs(10);
    let seven_nanos: Duration = Duration::from_nanos(7);
    let total: Duration = ten_seconds + seven_nanos;

    assert_eq!(total, Duration::new(10, 7));
    println!("{:?}", total);

}

fn duration_since()
{
    // Returns the amount of time elapsed from another instant to this one, 
    // or zero duration if that instant is later than this one.
   
    let now: Instant = Instant::now();
    slow_function(1, 100_000);
    let new_now: Instant = Instant::now();
    
    println!("{:?}", new_now.duration_since(now)); // 1.000288518s ~
    println!("{:?}", now.duration_since(new_now)); // 0ns
}

fn checked_duration_since()
{
    // Returns the amount of time elapsed from another instant to this one
    // or None if that instant is later than this one.
    
    let now: Instant  = Instant::now();
    sleep(Duration::new(1, 0));
    let new_now: Instant  = Instant::now();
    
    println!("{:?}", new_now.checked_duration_since(now)); // Some(1.000228368s)
    println!("{:?}", now.checked_duration_since(new_now)); // None 
}


fn slow_function(sec: u64, nanos: u32)
{
    sleep(Duration::new(sec, nanos));
}

fn elapsed_time()
{
    let now: Instant = Instant::now();

    slow_function(1, 100_000);


    let elapsed_time: Duration = now.elapsed();
    println!("Running slow_function() took {} seconds.", elapsed_time.as_secs());
    println!("Running slow_function() took {} nanoseconds.", elapsed_time.as_nanos());
}

pub fn test_all()
{
    create_Duration();
    
    // elapsed_time();
    // duration_since();
    // checked_duration_since();
}