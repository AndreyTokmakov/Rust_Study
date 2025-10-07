

mod simple_demo
{
    use std::sync::{Arc, Barrier};
    use std::thread;

    pub fn test()
    {
        let barrier: Arc<Barrier> = Arc::new(Barrier::new(5));
        let mut handles = vec![];

        for i in 0..5 {
            let b = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                println!("Thread {i} reached the barrier");
                b.wait(); // All threads wait here until the last one arrives
                println!("Thread {i} passed the barrier");
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    // Thread 0 reached the barrier
    // Thread 1 reached the barrier
    // Thread 3 reached the barrier
    // Thread 2 reached the barrier
    // Thread 4 reached the barrier
    // Thread 4 passed the barrier
    // Thread 3 passed the barrier
    // Thread 0 passed the barrier
    // Thread 1 passed the barrier
    // Thread 2 passed the barrier
}

pub fn test_all()
{
    simple_demo::test();
}