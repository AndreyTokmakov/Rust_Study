
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
    mpsc_receive_timeout::demo();
    // mpsc_example_1::demo();
    // mpsc_example_2::demo();
}