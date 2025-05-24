
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::{JoinHandle, ThreadId};

fn demo()
{
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>>  = Arc::clone(&counter);
        let handle: JoinHandle<()> = thread::spawn(move || {
            let threadId: ThreadId = thread::current().id();
            println!("Thread {:?} started", threadId);
            let mut num: MutexGuard<i32> = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn test_all()
{
    demo();
}