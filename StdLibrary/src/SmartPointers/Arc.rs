
mod basic
{
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::thread::JoinHandle;

    pub fn example()
    {
        let arcVec: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);

        for _ in 0..3 {
            let aClone: Arc<Vec<i32>> = Arc::clone(&arcVec);
            thread::spawn(move || {
                println!("{:?}", aClone);
            });
        }
    }

    pub fn example2()
    {
        let data: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3, 4, 5]);
        let mut handles: Vec<JoinHandle<()>> = vec![];

        for i in 0..3 {
            let data_clone: Arc<Vec<i32>> =  Arc::clone(&data);
            handles.push(thread::spawn(move || {
                println!("Thread {}: {:?}", i, *data_clone);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Original data still alive: {:?}", *data);
    }


    pub fn atomicCounter()
    {
        let counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let mut handles: Vec<JoinHandle<()>> = vec![];

        for _ in 0..10 {
            let counter: Arc<AtomicUsize> = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Final counter: {}", counter.load(Ordering::SeqCst)); // 10000
    }
}

mod consumer_producer
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
                q1.lock().unwrap().push(i);
                println!("Added: {}", i);
                thread::sleep(Duration::from_millis(50));
            }
        });

        // Consumer
        let q2: Arc<Mutex<Vec<i32>>>  = Arc::clone(&queue);
        let consumer = thread::spawn(move || {
            for _ in 0..5 {
                thread::sleep(Duration::from_millis(100));
                let mut q = q2.lock().unwrap();
                if let Some(val) = q.pop() {
                    println!("Consumed: {}", val);
                }
            }
        });

        producer.join().unwrap();
        consumer.join().unwrap();
    }
}

mod shared_access_to_config
{
    use std::sync::Arc;
    use std::thread;
    use std::thread::JoinHandle;

    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        max_connections: u32,
    }

    impl Config {
        fn new(host: &str, port: u16, max_connections: u32) -> Self {
            Config {
                host: host.to_string(),
                port,
                max_connections,
            }
        }
    }

    struct Worker {
        id: u32,
        config: Arc<Config>,
    }

    impl Worker
    {
        fn new(id: u32, config: Arc<Config>) -> Self {
            Worker { id, config }
        }

        fn work(&self) {
            println!("Worker {} connecting to {}:{}, max_conn: {}",
                     self.id, self.config.host, self.config.port, self.config.max_connections);
        }
    }

    pub fn run()
    {
        let config: Arc<Config> = Arc::new(Config::new("localhost", 8080, 100));
        let mut handles: Vec<JoinHandle<()>> = vec![];

        for i in 0..5 {
            let config_clone: Arc<Config> = Arc::clone(&config);
            handles.push(thread::spawn(move || {
                let worker = Worker::new(i, config_clone);
                worker.work();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Config still available: {:?}", *config);
    }
}

mod reference_count
{
    use std::sync::Arc;

    pub fn demo()
    {
        let v: Arc<i32> = Arc::new(42);

        println!("count = {}", Arc::strong_count(&v)); // 1

        let v2: Arc<i32> = Arc::clone(&v);
        println!("count = {}", Arc::strong_count(&v)); // 2

        drop(v2);
        println!("count = {}", Arc::strong_count(&v)); // 1
    }
}

mod shared_data_with_mutex
{
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread;
    use std::thread::JoinHandle;

    pub fn shared_counter()
    {
        let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let mut handles: Vec<JoinHandle<()>> = vec![];
        for _ in 0..10 {
            let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                let mut num: MutexGuard<i32> = counter.lock().unwrap();
                *num += 1;
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap()); //  ---> 10
    }
}


/**
    Arc<T> — потокобезопасный Rc
    Arc — Atomic Reference Counted — для многопоточности.
**/
pub fn test_all()
{
    // basic::example();
    // basic::example2();
    // basic::atomicCounter();

    // shared_access_to_config::run();
    // consumer_producer::example();
    // reference_count::demo();

    shared_data_with_mutex::shared_counter();
}