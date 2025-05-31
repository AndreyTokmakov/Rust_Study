
use std::{io, thread};
use std::fmt::format;
use std::thread::{sleep, ThreadId};
use std::time::Duration;
use std::sync::{Arc, Mutex, MutexGuard};


#[derive(Debug)]
struct User {
    name: String
}

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

fn pass_object_to_thread()
{
    let user = User { name: "sam".to_string() };

    thread::spawn(move || {
        println!("Hello from the first thread {}", user.name);
    }).join().unwrap();

    // println!("from the main thread {}", user.name);  <--- WILL NOT COMPILE
}

fn pass_object_to_multiple_threads()
{
    let original_user: Arc<User> = Arc::new(User { name: "Uncle Sam".to_string() });

    let user = original_user.clone();
    let t1 = thread::spawn(move || {
        println!("Hello from the first  thread '{}'", user.name);
    });

    let user = original_user.clone();
    let t2 = thread::spawn(move || {
        sleep(Duration::new(0, 500));
        println!("Hello from the second thread '{}'", user.name);
    });


    t1.join().unwrap();
    t2.join().unwrap();
}

fn pass_object_to_multiple_threads_modify()
{
    let shared_user = Arc::new(Mutex::new(User { name: String::from("Uncle Sam") }));

    let user: Arc<Mutex<User>> = shared_user.clone();
    let t1 = thread::spawn(move || {
        let threadIdStr: String = format!("{:?}", thread::current().id());
        let mut locked_user: MutexGuard<User> = user.lock().unwrap();
        //println!("Starting thread {:?} user: '{}'", threadIdStr.clone(),  locked_user.name);
        
        let new_val: String = locked_user.name.clone() + " [" +  &*threadIdStr + "]";
        locked_user.name = new_val.clone();
    });

    let user: Arc<Mutex<User>> = shared_user.clone();
    let t2 = thread::spawn(move || {
        sleep(Duration::new(0, 500));
        let threadIdStr: String = format!("{:?}", thread::current().id());
        let mut locked_user: MutexGuard<User> = user.lock().unwrap();
        //println!("Starting thread {:?} user: '{}'", threadIdStr.clone(),  locked_user.name);

        let new_val: String = locked_user.name.clone() + " [" +  &*threadIdStr + "]";
        locked_user.name = new_val.clone();
    });


    t1.join().unwrap();
    t2.join().unwrap();

    let user: Arc<Mutex<User>> = shared_user.clone();
    println!("Final value {:?}", user.lock().unwrap().name);
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

    // pass_object_to_thread();
    // pass_object_to_multiple_threads();
    pass_object_to_multiple_threads_modify();

    // create_thread_detached();
    // create_parallel_thread();
    // create_thead_join();
    // create_thead_builder();
}