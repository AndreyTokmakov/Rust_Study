
mod basic
{
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use std::thread;

    pub fn fetch_add()
    {
        // define the counter variable
        let counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(100));

        // increment the counter no lock or mutable borrow is necessary
        let old_value: u32 = counter.fetch_add(1, Ordering::SeqCst);

        println!("{}", old_value);
        println!("{}", counter.fetch_add(1, Ordering::SeqCst));
    }

    pub fn fetch_sub_thread()
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
}

mod basic_example_2
{
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::thread::JoinHandle;

    pub fn demo()
    {
        let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let mut workers: Vec<JoinHandle<()>> = vec![];

        for _ in 0..10 {
            let counter_clone: Arc<AtomicUsize> = Arc::clone(&counter);
            let handle: JoinHandle<()> = thread::spawn(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
            workers.push(handle);
        }

        for handle in workers {
            handle.join().unwrap();
        }
        println!("Final count: {}", counter.load(Ordering::SeqCst));
    }
}

mod Atomic_Bool
{
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Duration;

    pub fn busy_waiting()
    {
        let ready: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        // let ready_clone: Arc<AtomicBool> = ready.clone();
        let ready_clone: Arc<AtomicBool> = Arc::clone(&ready);

        let worker: thread::JoinHandle<()> = thread::spawn(move || {
            println!("Worker start");
            thread::sleep(Duration::from_secs(1));
            ready_clone.store(true, Ordering::Release);
            println!("Work is done");
        });

        while !ready.load(Ordering::Acquire) {
            // busy wait
        }

        println!("Worker end");
        let _ = worker.join();
    }
}

mod SpinLock
{
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::{hint, thread};
    
    
    pub fn demo()
    {
        let spinlock: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));
        let spinlock_clone: Arc<AtomicUsize> = Arc::clone(&spinlock);

        let thread = thread::spawn(move || {
            spinlock_clone.store(0, Ordering::Release);
        });

        // Wait for the other thread to release the lock
        while spinlock.load(Ordering::Acquire) != 0 {
            hint::spin_loop();
        }

        if let Err(panic) = thread.join() {
            println!("Thread had an error: {panic:?}");
        }
    }
}


// NOTE: https://doc.rust-lang.org/std/sync/atomic/index.html
pub fn test_all()
{
    // basic::fetch_add();
    // basic::fetch_sub_thread();
    basic_example_2::demo();
    // SpinLock::demo();
    // Atomic_Bool::busy_waiting();
}