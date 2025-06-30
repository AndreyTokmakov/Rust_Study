
mod basic
{
    use std::sync::Arc;
    use std::thread;

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



/**
    Arc<T> — потокобезопасный Rc
    Arc — Atomic Reference Counted — для многопоточности.
**/
pub fn test_all()
{
    // basic::example();
    consumer_producer::example();


}