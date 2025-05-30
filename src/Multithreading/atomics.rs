

use std::sync::{ atomic::{AtomicU32, Ordering}, Arc };
use std::sync::atomic::AtomicUsize;
use std::thread;

fn fetch_add()
{
    // define the counter variable
    let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(100));

    // increment the counter no lock or mutable borrow is necessary
    let old_value: u32 = counter.fetch_add(1, Ordering::SeqCst);

    println!("{}", old_value);
    println!("{}", counter.fetch_add(1, Ordering::SeqCst));
}

fn fetch_sub_thread()
{
    let counter: Arc<AtomicUsize>  = Arc::new(AtomicUsize::new(5));
    for _ in 0..10 {
        let val: Arc<AtomicUsize> = Arc::clone(&counter);
        thread::spawn(move || {
            let v: usize = val.fetch_add(1, Ordering::Relaxed);
            println!("{v:?}");
        });
    }
    
    // println!("{}", counter.fetch_add(1, Ordering::SeqCst));
}

pub fn test_all()
{
    // fetch_add();
    fetch_sub_thread();
}