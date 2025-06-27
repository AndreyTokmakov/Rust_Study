
use std::thread;
use std::sync::mpsc;

mod mpsc_simple_example
{
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let msg: String = String::from("Heyaa!");
            tx.send(msg.clone()).unwrap();
            println!("Sending: {}", msg);
        });

        let received: String = rx.recv().unwrap();
        println!("Received: {}", received);
    }
}


mod mpsc_several_messages_example
{
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let messages = vec!["one", "two", "three"];
            for msg in messages {
                tx.send(String::from(msg)).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        for received in rx {
            println!("Received: {}", received);
        }
    }
}

mod mpsc_multiple_producers
{
    use std::sync::mpsc;
    use std::thread;

    pub fn demo()
    {
        let (tx1, rx) = mpsc::channel();
        let tx2 = tx1.clone();

        thread::spawn(move || {
            tx1.send("Message 1").unwrap();
        });

        thread::spawn(move || {
            tx2.send("Message 2").unwrap();
        });

        for _ in 0..2 {
            println!("Received: {}", rx.recv().unwrap());
        }
    }
}

mod mpsc_receive_timeout
{
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx.send("Done").unwrap();
        });

        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(msg) => println!("Received: {}", msg),
            Err(err) => println!("Timeout: {}", err),
        }
    }
}

mod mpsc_try_recv
{
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx.send("Done").unwrap();
        });

        loop {
            match rx.try_recv() {
                Ok(msg) => {
                    println!("Received: {}", msg);
                    break;
                }
                Err(_) => {
                    println!("Waiting...");
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    }
}

mod mpsc_thread_poll_with_task_queue
{
    use std::sync::{ mpsc, Arc, Mutex };
    use std::thread;

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for i in 0..4 {
            let rx = Arc::clone(&rx);
            thread::spawn(move || loop {
                let task = rx.lock().unwrap().recv();
                match task {
                    Ok(n) => { 
                        println!("Thread {} working on task {}", i, n) 
                    },
                    Err(_) => break,
                }
            });
        }

        for i in 1..=10 {
            tx.send(i).unwrap();
        }
    }
}

mod mpsc_two_way_communication
{
    use std::sync::mpsc;
    use std::thread;

    pub fn demo()
    {
        // Канал от главного потока к воркеру
        let (tx1, rx1) = mpsc::channel();

        // Канал от воркера к главному потоку
        let (tx2, rx2) = mpsc::channel();

        thread::spawn(move || {
            while let Ok(msg) = rx1.recv() {
                println!("Worker received:: {}", msg);
                tx2.send(format!("Response'{}'", msg)).unwrap();
            }
        });

        tx1.send("Request 1").unwrap();
        let reply = rx2.recv().unwrap();
        println!("Main thread gets: {}", reply);
    }
}

mod mpsc_parallel_tasks_processing_with_results
{
    use std::sync::mpsc;
    use std::thread;

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();

        for i in 0..5 {
            let tx = tx.clone();
            thread::spawn(move || {
                let result = i * i;
                tx.send((i, result)).unwrap();
            });
        }

        drop(tx); // закрываем отправитель, чтобы for внизу завершился
        for (i, result) in rx {
            println!("Квадрат {} = {}", i, result);
        }
    }
}

mod mpsc_graceful_shutdown
{
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    enum Command {
        Work(u32),
        Quit,
    }

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            for cmd in rx {
                match cmd {
                    Command::Work(n) => println!("Working on task: {}", n),
                    Command::Quit => {
                        println!("Stopping thread");
                        break;
                    }
                }
            }
        });

        tx.send(Command::Work(1)).unwrap();
        tx.send(Command::Work(2)).unwrap();
        tx.send(Command::Quit).unwrap();

        handle.join().unwrap();
    }
}


mod mpsc_separate_thread_logger_and_worker
{
    use std::sync::{mpsc, Arc, Mutex, MutexGuard};
    use std::thread;

    pub fn demo()
    {
        let logQueue: Arc<Mutex<Vec<String>>>  = Arc::new(Mutex::new(vec![]));
        let (tx, rx) = mpsc::channel();

        // Поток логгера
        let logs: Arc<Mutex<Vec<String>>> = Arc::clone(&logQueue);
        thread::spawn(move || {
            for msg in rx {
                logs.lock().unwrap().push(msg);
            }
        });

        // Несколько логгируемых сообщений из разных потоков
        for i in 0..5 {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(format!("Message from thread {}", i)).unwrap();
            });
        }

        drop(tx); // закрываем канал

        // Подождём завершения логгера (плохо, но просто)
        thread::sleep(std::time::Duration::from_millis(500));

        let log: MutexGuard<Vec<String>> = logQueue.lock().unwrap();
        for entry in log.iter() {
            println!("Log: {}", entry);
        }
    }
}

mod mpsc_received_channel_with_filter
{
    use std::sync::mpsc::{self, Receiver};
    use std::thread;
    
    struct FilteredReceiver {
        inner: Receiver<i32>,
    }

    impl FilteredReceiver
    {
        fn new(rx: Receiver<i32>) -> Self {
            Self { inner: rx }
        }

        fn recv_even(&self) -> Option<i32> {
            while let Ok(value) = self.inner.recv() {
                if value % 2 == 0 {
                    return Some(value);
                }
            }
            None
        }
    }

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();
        let filtered: FilteredReceiver = FilteredReceiver::new(rx);

        thread::spawn(move || {
            for i in 1..10 {
                tx.send(i).unwrap();
            }
        });

        while let Some(val) = filtered.recv_even() {
            println!("Получено чётное число: {}", val);
        }
    }
}

mod mpsc_example_1
{
    use std::sync::mpsc;
    use std::thread;

    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();

        let sender = thread::spawn(move || {
            tx.send("Hello, thread".to_owned()).expect("Unable to send on channel");
        });

        let receiver = thread::spawn(move || {
            let value: String = rx.recv().expect("Unable to receive from channel");
            println!("Receiver: {value}");
        });

        sender.join().expect("The sender thread has panicked");
        receiver.join().expect("The receiver thread has panicked");
    }
}

mod mpsc_example_2
{
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    
    pub fn demo()
    {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for i in 0..4 {
            let loc_rx = Arc::clone(&rx);
            thread::spawn(move || {
                loop {
                    let job = loc_rx.lock().unwrap().recv();
                    match job {
                        Ok(data) => println!("Поток {} получил: {}", i, data),
                        Err(_) => break,
                    }
                }
            });
        }

        for i in 0..10 {
            tx.send(i).unwrap();
        }
        drop(tx); // закрываем канал
    }
}



pub fn test_all()
{
    // mpsc_simple_example::demo();
    // mpsc_several_messages_example::demo();
    // mpsc_multiple_producers::demo();
    // mpsc_receive_timeout::demo();
    // mpsc_try_recv::demo();
    // mpsc_thread_poll_with_task_queue::demo();
    // mpsc_two_way_communication::demo();
    // mpsc_parallel_tasks_processing_with_results::demo();
    // mpsc_graceful_shutdown::demo();
    // mpsc_separate_thread_logger_and_worker::demo();
    mpsc_received_channel_with_filter::demo();
    // mpsc_example_1::demo();
    // mpsc_example_2::demo();
}