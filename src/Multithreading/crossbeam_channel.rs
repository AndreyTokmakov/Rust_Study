
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


pub fn test_all()
{
    basic::demo();
}