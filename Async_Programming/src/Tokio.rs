

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



mod basic_examples
{
    use futures::future::join_all;
    use std::time::Duration;
    use chrono::Local;
    use futures::future;

    const DATE_FORMAT: &'static str = "[%Y-%m-%d %H:%M:%S.%3f]";


    async fn task(n: u64, delay_sec: u64)
    {
        println!("{} Starting task {n}", Local::now().format(DATE_FORMAT));
        tokio::time::sleep(Duration::from_secs(delay_sec)).await;
        println!("{} Task {n} completed", Local::now().format(DATE_FORMAT));
    }

    #[tokio::main]
    pub async fn test_bad()
    {
        task(1, 1).await;
        task(2, 2).await;
        task(3, 3).await;

        // [2026-01-08 16:28:05.199] Starting task 1
        // [2026-01-08 16:28:06.200] Task 1 completed
        // [2026-01-08 16:28:06.200] Starting task 2
        // [2026-01-08 16:28:08.202] Task 2 completed
        // [2026-01-08 16:28:08.202] Starting task 3
        // [2026-01-08 16:28:11.203] Task 3 completed
    }

    #[tokio::main]
    pub async fn test_good()
    {
        let mut futures = Vec::new();
        for n in 1..=5 {
            futures.push(task(n, n));
        }
        let joined_future = future::join_all(futures);
        joined_future.await;

        // [2026-01-08 16:31:30.357] Starting task 1
        // [2026-01-08 16:31:30.357] Starting task 2
        // [2026-01-08 16:31:30.357] Starting task 3
        // [2026-01-08 16:31:30.357] Starting task 4
        // [2026-01-08 16:31:30.357] Starting task 5
        // [2026-01-08 16:31:31.358] Task 1 completed
        // [2026-01-08 16:31:32.358] Task 2 completed
        // [2026-01-08 16:31:33.359] Task 3 completed
        // [2026-01-08 16:31:34.358] Task 4 completed
        // [2026-01-08 16:31:35.359] Task 5 completed
    }
}

mod practical_examples_1
{
    use tokio::time::{sleep, Duration};
    use std::error::Error;

    async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
        // In a real application, you would use request or similar
        // Here we're simulating network delays
        println!("Fetching: {}", url);

        // Simulate different download times
        let delay = match url {
            "https://example.com" => 2,
            "https://rust-lang.org" => 3,
            _ => 1,
        };

        sleep(Duration::from_secs(delay)).await;
        Ok(format!("Content from {}: [... data ...]", url))
    }

    #[tokio::main]
    async fn download_sites() -> Result<(), Box<dyn Error>>
    {
        let urls = vec![
            "https://example.com",
            "https://rust-lang.org",
            "https://docs.rs",
        ];

        let mut handles = vec![];

        // Start all downloads concurrently
        for url in urls {
            let handle = tokio::spawn(async move {
                match fetch_url(url).await {
                    Ok(content) => println!("Downloaded: {}", content),
                    Err(e) => eprintln!("Error downloading {}: {}", url, e),
                }
            });

            handles.push(handle);
        }

        // Wait for all downloads to complete
        for handle in handles {
            handle.await?;
        }

        println!("All downloads complete!");
        Ok(())
    }

    pub fn concurrent_downloads()
    {
        let _ = download_sites();

        // Fetching: https://example.com
        // Fetching: https://rust-lang.org
        // Fetching: https://docs.rs
        // Downloaded: Content from https://docs.rs: [... data ...]
        // Downloaded: Content from https://example.com: [... data ...]
        // Downloaded: Content from https://rust-lang.org: [... data ...]
        // All downloads complete!
    }
}

mod practical_examples_2
{
    use std::{env, fs};
    use std::path::PathBuf;
    use tokio::fs::File;
    use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

    #[tokio::main]
    async fn read_write_file_async() -> io::Result<()>
    {
        let file_path: PathBuf = env::current_dir().unwrap().join("resources/test_files/async_test.txt");
        let mut file: File = File::create(&file_path).await?;

        // Write data asynchronously
        file.write_all(b"Hello, async Rust!").await?;
        file.write_all(b"This file was written asynchronously.").await?;

        // Make sure data is flushed to disk
        file.flush().await?;

        println!("File written successfully");

        let mut file: File = File::open(&file_path).await?;

        // Read the content into a string
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).await?;

        println!("File contents: '{}'", contents);

        // Clean up
        tokio::fs::remove_file(&file_path).await?;
        println!("File removed");

        Ok(())
    }

    pub fn concurrent_read_write_file_async()
    {
        let _ = read_write_file_async();

        // File written successfully
        // File contents: 'Hello, async Rust!This file was written asynchronously.'
        // File removed
    }
}

pub fn tokio_tests()
{
    // basic_examples::test_bad();
    // basic_examples::test_good();

    // tokio_examples

    // practical_examples_1::concurrent_downloads();
    practical_examples_2::concurrent_read_write_file_async();
}