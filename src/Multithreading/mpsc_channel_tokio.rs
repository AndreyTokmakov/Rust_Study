
mod basic
{
    use tokio::sync::mpsc;
    use tokio::time::{sleep, Duration};

    #[tokio::main]
    pub async fn demo()
    {
        let (tx, mut rx) = mpsc::channel(32);

        for i in 0..3 {
            let tx = tx.clone();
            tokio::spawn(async move {
                tx.send(format!("Hello from {}", i)).await.unwrap();
            });
        }

        drop(tx); // close the channel to end the loop

        while let Some(msg) = rx.recv().await {
            println!("Received: {}", msg);
        }
    }
}


pub fn test_all()
{
    basic::demo();
}