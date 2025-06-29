
mod simple_example
{
    use tokio::sync::oneshot;

    async fn some_computation() -> String {
        "MESSAGE".to_string()
    }

    #[tokio::main]
    pub async fn run()
    {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let res = some_computation().await;
            tx.send(res).unwrap();
        });

        // Do other work while the computation is happening in the background

        // Wait for the computation result
        let res: String = rx.await.unwrap();
        
        println!("{}", res);
    }
}

mod using_thread
{
    async fn some_computation() -> String {
        "MESSAGE".to_string()
    }

    #[tokio::main]
    pub async fn run()
    {
        let join_handle = tokio::spawn(async move {
            some_computation().await
        });
        
        let res: String = join_handle.await.unwrap();
        println!("{}", res);
    }
}

mod mscp_oneshot_synchronisation
{
    use tokio::sync::{oneshot, mpsc};
    use Command::Increment;

    enum Command {
        Increment,
        // Other commands can be added here
    }

    #[tokio::main]
    pub async fn run()
    {
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);

        // Spawn a task to manage the counter
        tokio::spawn(async move {
            let mut counter: u64 = 0;
            while let Some((cmd, response)) = cmd_rx.recv().await {
                match cmd {
                    Increment => {
                        let prev = counter;
                        counter += 1;
                        response.send(prev).unwrap();
                    }
                }
            }
        });

        let mut join_handles = vec![];

        // Spawn tasks that will send the increment command.
        for _ in 0..10 {
            let cmd_tx = cmd_tx.clone();
            join_handles.push(tokio::spawn(async move {
                let (resp_tx, resp_rx) = oneshot::channel();
                cmd_tx.send((Increment, resp_tx)).await.ok().unwrap();
                let res = resp_rx.await.unwrap();
                println!("previous value = {}", res);
            }));
        }

        // Wait for all tasks to complete
        for join_handle in join_handles.drain(..) {
            join_handle.await.unwrap();
        }
    }
}

pub fn test_all()
{
    // simple_example::run();
    // using_thread::run();
    mscp_oneshot_synchronisation::run();
}