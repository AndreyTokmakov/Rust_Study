
mod basic
{
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread;
    
    pub fn example()
    {
        let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num: MutexGuard<i32> = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result = {}", *counter.lock().unwrap()); // 10
    }
}


/**
    Mutex<T> — защита от гонки данных
    Allows mutation even with an immutable reference.
    ❗ Check for multiple mutations - at runtime, not at compile time.
**/
pub fn test_all()
{
    basic::example();

}