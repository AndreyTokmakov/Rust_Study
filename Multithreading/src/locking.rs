

mod write_guard_tests
{
    use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
    use std::thread;
    use std::time::{Duration, Instant};

    pub fn read_write_guard_bug()
    {
        let data: Arc<RwLock<i32>> = Arc::new(RwLock::new(1000));
        let cloned_data: Arc<RwLock<i32>> = Arc::clone(&data);
        
        let read_guard: RwLockReadGuard<i32> = data.read().unwrap();
        println!("Main thread acquired read lock. Data = {:?}", read_guard);

        // INFO: Right place to drop the lock guard
        drop(read_guard);
        
        let handle = thread::spawn(move || {
            let start_time: Instant = Instant::now();

            println!("Other thread trying to acquire write lock...");
            let mut write_guard: RwLockWriteGuard<i32> = cloned_data.write().unwrap();
            let duration: Duration = start_time.elapsed();
            
            *write_guard += 1;
            println!("Other thread have acquired write lock after waiting {:?} seconds.", duration);
        });
        
        thread::sleep(Duration::from_secs(3));
        
        // INFO: 'drop(read_guard)' have to be called earlier
        // drop(read_guard); 
        
        let result: RwLockReadGuard<i32> = data.read().unwrap(); 
        println!("Result = {:?}", *result);
    }
    
    // Main thread acquired read lock. Data = 1000
    // Other thread trying to acquire write lock...
    // Result = 1000
    // Other thread have acquired write lock after waiting 3.000096511s seconds.
}

mod multiple_reader_one_writer
{
    use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
    use std::thread;
    use std::time::{Duration, Instant};

    pub fn demo()
    {
        let data: Arc<RwLock<i32>> = Arc::new(RwLock::new(0));
        let mut reader_collection: Vec<thread::JoinHandle<()>> = Vec::new();

        for reader in 1..3 {
            let data: Arc<RwLock<i32>> = Arc::clone(&data);
            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                // read() Locks this RwLock with shared read access
                let val: std::sync::RwLockReadGuard<'_, i32> = data.read().unwrap();
                println!("Reader {}:  Value = {}", reader, *val);
            });
            reader_collection.push(handle);
        }

        let writer: thread::JoinHandle<()> = {
            let data: Arc<RwLock<i32>> = Arc::clone(&data);
            thread::spawn(move || {
                // write() Locks this RwLock with exclusive write access
                let mut val: std::sync::RwLockWriteGuard<'_, i32> = data.write().unwrap();
                *val += 1;
                print!("Writer: Value = {}", *val);
            })
        };

        for handle in reader_collection {
            handle.join().unwrap();
        }
        writer.join().unwrap();
    }
}

pub fn test_all()
{
    // write_guard_tests::read_write_guard_bug();
    multiple_reader_one_writer::demo();
}