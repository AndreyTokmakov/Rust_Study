
use std::{io, thread};
use std::thread::{sleep, ThreadId};
use std::time::Duration;


fn get_available_parallelism() -> io::Result<()>
{
    let count = thread::available_parallelism()?.get();
    assert!(count >= 1_usize);
    Ok(())
}

fn get_parallelism()
{
    let count = thread::available_parallelism().unwrap().get();
    print!("{count}");
}

fn create_thread_detached()
{
    thread::spawn(move || {
        // some work here
    });
}

fn create_thead_join()
{
    let computation = thread::spawn(move || {
        let threadId: ThreadId = thread::current().id();
        println!("Thread {:?} started", threadId);
        sleep(Duration::new(1, 0));
        println!("Thread {:?} completed", threadId);
        12345
    });

    let result = computation.join().unwrap();
    println!("Result = {result}");
}

fn create_parallel_thread()
{
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();
}



fn create_thead_builder()
{
    let thread = thread::Builder::new().name("thread1".to_string()).spawn(move || {
        println!("Hello, world!");
    });
    
}

pub fn test_all()
{
    // get_parallelism();
    // create_thread_detached();
    create_parallel_thread();
    // create_thead_join();
    // create_thead_builder();
}