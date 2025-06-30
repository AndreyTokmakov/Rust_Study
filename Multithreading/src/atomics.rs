
mod basic
{
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
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

    SpinLock::demo();
}