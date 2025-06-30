

mod basic
{      
    use std::sync::{Arc, Mutex, Condvar, MutexGuard};
    use std::thread;
    use chrono::Local;
    
    const DATE_FORMAT: &'static str = "[%Y-%m-%d %H:%M:%S.%3f]";

    pub fn demo()
    {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair1 = Arc::clone(&pair);
        let pair2 = Arc::clone(&pair);
        
        let t1 = thread::spawn(move || {
            println!("{} T1: Started", Local::now().format(DATE_FORMAT));
            let (lock, cvar) = &*pair1;
            let mut done: MutexGuard<bool> = lock.lock().unwrap();

            thread::sleep(std::time::Duration::from_secs(1));
            println!("{} T1: Set CondVar and Notify!", Local::now().format(DATE_FORMAT));
            
            *done = true;
            cvar.notify_one();
        });

        let t2 = thread::spawn(move || {
            println!("{} T2: Started - Wait for the thread to start up....", Local::now().format(DATE_FORMAT));
            let (lock, cvar) = &*pair2;
            let mut done: MutexGuard<bool> = lock.lock().unwrap();
            while !*done {
                done = cvar.wait(done).unwrap();
            }
            println!("{} T2: Done", Local::now().format(DATE_FORMAT));
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}

mod wait_timeout_ms
{
    use std::sync::{Arc, Mutex, Condvar, MutexGuard};
    use std::thread;
    use std::time::Duration;
    use chrono::Local;

    const DATE_FORMAT: &'static str = "[%Y-%m-%d %H:%M:%S.%3f]";

    pub fn demo()
    {
        let orig = Arc::new((Mutex::new(false), Condvar::new()));
        let pair1 = Arc::clone(&orig);
        let pair2 = Arc::clone(&orig);

        let t1 = thread::spawn(move || {
            println!("{} T1: Started", Local::now().format(DATE_FORMAT));
            let (lock, cvar) = &*pair1;
            let mut done: MutexGuard<bool> = lock.lock().unwrap();

            {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("{} T1: Set to False CondVar and Notify!", Local::now().format(DATE_FORMAT));

                *done = false;
                cvar.notify_one();
            }

            {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("{} T1: Set to TRUE CondVar and Notify!", Local::now().format(DATE_FORMAT));

                *done = true;
                cvar.notify_one();
            }
        });

        let t2 = thread::spawn(move || {
            println!("{} T2: Started - Wait for the thread to start up....", Local::now().format(DATE_FORMAT));
            let (lock, cvar) = &*pair2;
            let mut done: MutexGuard<bool> = lock.lock().unwrap();

            loop {
                let result = cvar.wait_timeout(done, Duration::from_millis(10)).unwrap();
                println!("{} T2: wait_timeout done", Local::now().format(DATE_FORMAT));
                done = result.0;
                if *done == true {
                    println!("{} T2: done == True --> break", Local::now().format(DATE_FORMAT));
                    break
                }
                else {
                    println!("{} T2: done == False --> continue", Local::now().format(DATE_FORMAT));
                }
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}

pub fn test_all()
{
    // basic::demo();
    wait_timeout_ms::demo();
}