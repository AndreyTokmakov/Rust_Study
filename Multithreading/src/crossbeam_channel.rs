
mod basic
{
    use crossbeam::channel;
    use std::thread;
    use std::time::Duration;

    pub fn demo()
    {
        let (tx, rx) = channel::unbounded();

        // Несколько производителей
        for i in 0..3 {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(format!("Сообщение от потока {}", i)).unwrap();
            });
        }

        // Несколько потребителей
        for id in 0..2 {
            let rx = rx.clone();
            thread::spawn(move || {
                while let Ok(msg) = rx.recv() {
                    println!("Поток {} получил: {}", id, msg);
                }
            });
        }

        thread::sleep(Duration::from_secs(1));
        // Канал автоматически закроется, когда все tx будут drop.
    }
}

mod basic_receive_in_thread
{
    use crossbeam::channel;
    use std::thread;

    pub fn demo()
    {
        let (tx, rx) = channel::unbounded();

        // Worker pool
        for id in 0..4 {
            let rx = rx.clone();
            thread::spawn(move || {
                for task in rx.iter() {
                    println!("Thread {} is processing {}", id, task);
                }
            });
        }

        // Generating tasks
        for task_id in 0..10 {
            tx.send(format!("Task {}", task_id)).unwrap();
        }

        drop(tx); // close channel
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

mod receive_select
{
    use crossbeam::channel::{unbounded, select};
    use std::thread;

    pub fn demo()
    {
        let (a_tx, a_rx) = unbounded();
        let (b_tx, b_rx) = unbounded();

        thread::spawn(move || {  a_tx.send("From A").unwrap(); });
        thread::spawn(move || {  b_tx.send("From B").unwrap(); });

        for _ in 0..2 {
            select! {
                recv(a_rx) -> msg => println!("Received from A: {}", msg.unwrap()),
                recv(b_rx) -> msg => println!("Received from B: {}", msg.unwrap()),
            }
        }
    }
}

mod receive_timeout
{
    use crossbeam::channel;
    use std::time::Duration;
    use std::thread;

    pub fn demo()
    {
        let (tx, rx) = channel::unbounded();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx.send("Done").unwrap();
        });

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => println!("Received: {}", msg),
            Err(_) => println!("Timeout!"),
        }
    }
}

mod receive_processing_with_iter
{
    use crossbeam::channel;
    use std::thread;

    pub fn demo()
    {
        let (tx, rx) = channel::unbounded();
        
        for i in 0..3 {
            let tx = tx.clone();
            thread::spawn(move || {
                for j in 0..5 {
                    tx.send(format!("{} - {}", i, j)).unwrap();
                }
            });
        }

        drop(tx); // closing the last sender

        // The consumer will automatically terminate when the channel closes.
        for msg in rx.iter() {
            println!("Received: {}", msg);
        }

        println!("All data has been processed.");
    }
}

mod logger_handle_parallel_thread
{
    use crossbeam::channel;
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread;
    
    pub fn demo()
    {
        let (tx, rx) = channel::unbounded();
        let logs: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

        println!("Launching workers");
        for i in 0..3 {
            let rx = rx.clone();
            let log_clone: Arc<Mutex<Vec<String>>> = Arc::clone(&logs);
            thread::spawn(move || {
                for msg in rx.iter() {
                    thread::sleep(std::time::Duration::from_nanos(25));
                    let mut logs: MutexGuard<Vec<String>> = log_clone.lock().unwrap();
                    logs.push(format!("Thread {}: {}", i, msg));

                }
            });
        }

        println!("Sending data");
        for n in 0..10 {
            tx.send(format!("Message {}", n)).unwrap();
        }
        drop(tx);

        println!("Before sleeping");
        thread::sleep(std::time::Duration::from_millis(250));

        println!("Logs collected:");
        for line in logs.lock().unwrap().iter() {
            println!("{}", line);
        }
    }
}


pub fn test_all()
{
    // basic::demo();
    // basic_receive_in_thread::demo();
    // receive_select::demo();
    // receive_timeout::demo();
    // receive_processing_with_iter::demo();
    logger_handle_parallel_thread::demo();
}