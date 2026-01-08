#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};




mod tokio_examples
{
    use tokio::task::JoinHandle;
    use tokio::time::sleep;
    use std::time::Duration;
    use chrono::Local;
    const DATE_FORMAT: &'static str = "[%Y-%m-%d %H:%M:%S.%3f]";

    async fn fetch_data() -> String {
        // Simulating a network request
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        "Data from server".to_string()
    }

    async fn process() {
        let data: String = fetch_data().await;
        println!("{} Received: {}", Local::now().format(DATE_FORMAT), data);
    }

    async fn say_hello(name: &str, delay: u64) {
        sleep(Duration::from_secs(delay)).await;
        println!("{} Hello, {}!", Local::now().format(DATE_FORMAT), name);
    }

    #[tokio::main]
    pub async fn run()
    {
        // These will run concurrently
        let task1: JoinHandle<()> = tokio::spawn(say_hello("Alice", 1));
        let task2: JoinHandle<()> = tokio::spawn(process());
        let task3: JoinHandle<()> = tokio::spawn(say_hello("Charlie", 2));

        // Wait for all tasks to complete
        let _ = tokio::join!(task1, task2, task3);

        println!("* * * * All tasks completed * * * *");

        // [2026-01-08 10:45:02.605] Hello, Alice!
        // [2026-01-08 10:45:02.605] Received: Data from server
        // [2026-01-08 10:45:03.604] Hello, Charlie!
        // * * * * All tasks completed * * * *
    }
}


fn main()
{
    tokio_examples::run();
}
