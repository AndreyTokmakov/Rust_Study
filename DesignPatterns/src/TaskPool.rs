
mod simple
{
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;

    pub fn demo() 
    {
        let (tx, rx) = mpsc::channel();
        let rx  = Arc::new(Mutex::new(rx));

        for i in 0..4 {
            let rx = Arc::clone(&rx);
            thread::spawn(move || {
                loop {
                    let job = rx.lock().unwrap().recv();
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
    simple::demo();
}