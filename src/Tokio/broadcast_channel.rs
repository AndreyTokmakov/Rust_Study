

mod multiple_subscribers
{
    use tokio::sync::broadcast;
    use tokio::task::JoinHandle;
    use tokio::time::{sleep, Duration};

    #[tokio::main]
    pub async fn run()
    {
        //let mut workers = Vec::new();
        let (tx, _) = broadcast::channel(16);
    
        for id in 0..3 {
            let mut rx = tx.subscribe();
            let handle = tokio::spawn(async move {
                println!("Subscriber {} created", id);
                while let Ok(msg) = rx.recv().await {
                    println!("Subscriber {} got: {}", id, msg);
                }
            });
            // workers.push(handle);
        }

        sleep(Duration::from_millis(100)).await;
        println!("Publisher sends messages...");
        
        for i in 0..5 {
            tx.send(format!("Message {}", i)).unwrap();
            sleep(Duration::from_millis(100)).await;
        }

        /*for handle in workers {
            handle.await.unwrap()
        }*/

        sleep(Duration::from_millis(500)).await;
    }
}
mod multiple_publishers
{
    use tokio::sync::broadcast;
    use tokio::time::{sleep, Duration};

    #[tokio::main]
    pub async fn run()
    {
        let (tx, _) = broadcast::channel(16);

        for id in 0..2 {
            let mut rx = tx.subscribe();
            tokio::spawn(async move {
                println!("Subscriber {} started", id);
                while let Ok(msg) = rx.recv().await {
                    println!("Subscriber {} got: {}", id, msg);
                }
            });
        }

        // Multiple publishers
        for j in 0..3 {
            let tx = tx.clone();
            tokio::spawn(async move {
                for i in 0..3 {
                    tx.send(format!("Publisher {} -> {}", j, i)).unwrap();
                    sleep(Duration::from_millis(50)).await;
                }
            });
        }

        sleep(Duration::from_millis(500)).await;
    }
}

mod handling_lagged_errors
{
    // If a receiver is too slow and misses messages (buffer was overwritten), it will get:
    //   Err(tokio::sync::broadcast::error::RecvError::Lagged(n))

    use tokio::sync::broadcast;
    use tokio::sync::broadcast::Receiver;
    use tokio::time::{sleep, Duration};

    #[tokio::main]
    pub async fn run()
    {
        let (tx, _) = broadcast::channel(3); // small buffer
        let mut rx: Receiver<String> = tx.subscribe();

        tokio::spawn(async move {
            for i in 0..10 {
                tx.send(format!("Message {}", i)).unwrap();
                sleep(Duration::from_millis(30)).await;
            }
        });

        // Simulate slow subscriber
        sleep(Duration::from_millis(200)).await;

        loop {
            match rx.recv().await {
                Ok(msg) => println!("Got: {}", msg),
                Err(tokio::sync::broadcast::error::RecvError::Lagged(count)) => {
                    println!("Lagged and missed {} messages!", count);
                },
                Err(_) => break,
            }
        }
        
        // Lagged and missed 3 messages!
        // Got: Message 3
        // Got: Message 4
        // Got: Message 5
        // Got: Message 6
        // Got: Message 7
        // Got: Message 8
        // Got: Message 9
    }
}

/// A broadcast channel is a multi-producer, multi-subscriber (publish-subscribe) channel:
///  - Each send is received by all active subscribers.
///  - Each subscriber (Receiver) has its own buffer position.
///  - If a receiver lags too far behind (buffer overflows), it will see an Err(RecvError::Lagged(n)).
/// 
/// This is different from mpsc where each message is received by only one receiver.
/// 
/// Summary: tokio::broadcast key points
///  - Multiple producers, multiple subscribers (fan-out).
///  - Each subscriber gets all messages, independent of each other.
///  - If a subscriber lags, it will see Lagged(n) and knows how many messages were missed.
///  - Useful for event buses, logging, or signaling multiple tasks.

pub fn test_all()
{
    // multiple_subscribers::run();
    // multiple_publishers::run();
    handling_lagged_errors::run();
}