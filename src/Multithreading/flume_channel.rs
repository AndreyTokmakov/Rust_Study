
mod basic
{
    use flume;
    use std::thread;
    use std::time::Duration;

    pub fn demo()
    {
        let (tx, rx) = flume::bounded(5); // или unbounded()

        // Несколько производителей
        for i in 0..3 {
            let tx = tx.clone();
            thread::spawn(move || {
                for j in 0..5 {
                    tx.send(format!("Поток {} -> {}", i, j)).unwrap();
                }
            });
        }

        // Несколько потребителей
        for id in 0..2 {
            let rx = rx.clone();
            thread::spawn(move || {
                while let Ok(msg) = rx.recv() {
                    println!("Consumer {} получил: {}", id, msg);
                }
            });
        }

        thread::sleep(Duration::from_millis(500));
    }
}



pub fn test_all()
{
    basic::demo();
}