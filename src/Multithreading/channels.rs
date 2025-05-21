
use std::thread;
use std::sync::mpsc;

fn mpsc_example()
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

pub fn test_all()
{
    mpsc_example();
}