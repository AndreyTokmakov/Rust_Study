
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::{sleep, JoinHandle, ThreadId};
use std::time::Duration;

fn simple_example()
{
    // defining the counter variable
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    println!("counter: {}", *counter.lock().unwrap());
    
    // lock the mutex to borrow it is automatically released when the borrow ends
    let mut counter_lock: MutexGuard<i32> = counter.lock().unwrap();
    *counter_lock = *counter_lock + 1;

    println!("counter: {}", *counter.lock().unwrap());
}


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


struct User
{
    name: String,
    email: String,
}

fn lock_multiple_threads()
{
    let user_original = Arc::new(Mutex::new(
        User { name: String::from("sam"), email: "andtokm@yandex.ru".to_string() }));

    let user: Arc<Mutex<User>> = user_original.clone();
    let t1 = thread::spawn(move || {
        let mut locked_user = user.lock().unwrap();
        locked_user.name = String::from("sam");
        // После того как «locked_user» выйдет из области видимости, мьютекс снова разблокируется.
        // Чтобы разблокировать его явно, применяется «drop(locked_user)».
    });

    let user: Arc<Mutex<User>> = user_original.clone();
    let t2 = thread::spawn(move || {
        sleep(Duration::from_millis(10));

        // Выведется «Hello sam»
        let locked_user: MutexGuard<User> = user.lock().unwrap();
        println!("Hello {} {}", locked_user.name, locked_user.email);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}


mod consumer_producer_demo
{
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    pub fn example()
    {
        let queue: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

        // Producer
        let q1: Arc<Mutex<Vec<i32>>> = Arc::clone(&queue);
        let producer = thread::spawn(move || {
            for i in 0..5 {

                let mut queue = q1.lock().unwrap();
                queue.push(i);
                
                println!("Added: {}", i);
                thread::sleep(Duration::from_millis(50));
            }
        });

        // Consumer
        let q2: Arc<Mutex<Vec<i32>>>  = Arc::clone(&queue);
        let consumer = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_millis(100));
                let mut queue = q2.lock().unwrap();
                if let Some(val) = queue.pop() {
                    println!("Consumed: {}", val);
                }
            }
        });

        producer.join().unwrap();
        consumer.join().unwrap();
    }
}




pub fn test_all()
{
    // simple_example();
    // demo();
    // lock_multiple_threads();

    consumer_producer_demo::example();
}