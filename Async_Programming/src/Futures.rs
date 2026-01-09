
// https://www.compilenrun.com/docs/language/rust/rust-async-programming/rust-future-trait/


mod utils
{
    use reqwest::{Error, Response};

    pub async fn fetch_data(url: &str) -> Result<String, Error>
    {
        // Asynchronous code here
        let response: Response = reqwest::get(url).await?;
        let body: String = response.text().await?;
        Ok(body)
    }
}

mod example_1
{
    use crate::Futures::utils::fetch_data;
    use futures::join;

    #[tokio::main]
    async fn call_api_concurrent()
    {
        let future1 = fetch_data("https://httpbin.org/uuid");
        let future2 = fetch_data("https://httpbin.org/ip");

        let (result1, result2) = join!(future1, future2);

        println!("Result 1: {:?}", result1);
        println!("Result 2: {:?}", result2);
    }

    pub fn run_multiple_futures_concurrently()
    {
        let _ = call_api_concurrent();

        // Result 1: Ok("{\n  \"uuid\": \"6b7663aa-543e-4da5-afe8-37c82525ad12\"\n}\n")
        // Result 2: Ok("{\n  \"origin\": \"218.17.7.229\"\n}\n")
    }
}

mod example_2
{
    use crate::Futures::utils::fetch_data;
    use futures::FutureExt;
    use futures::select;

    #[tokio::main]
    async fn call_api_concurrent()
    {
        let future1 = fetch_data("https://httpbin.org/uuid");
        let future2 = fetch_data("https://httpbin.org/ip");

        select! {
            result = future1.fuse() => {
                println!("Future 1 completed first: {:?}", result);
            }
            result = future2.fuse() => {
                println!("Future 2 completed first: {:?}", result);
            }
        }
    }

    pub fn selecting_between_futures()
    {
        let _ = call_api_concurrent();

        // Future 1 completed first: Ok("{\n  \"uuid\": \"3629f75f-3a60-451d-817b-36c0851f17c2\"\n}\n")
        //   or
        // Future 2 completed first: Ok("{\n  \"origin\": \"218.17.7.229\"\n}\n")
    }
}


mod http_request_async
{
    use anyhow::Result;

    #[tokio::main]
    async fn fetch_url(endpoint: &str) -> Result<()>
    {
        let response = reqwest::get(endpoint).await?;
        println!("Status: {} \nHeaders:\n{:#?}", response.status(), response.headers());

        let payload: String = response.text().await?;
        println!("Body:\n{}", payload);
        Ok(())
    }

    pub fn run() {
        let _ = fetch_url("https://httpbin.org/get");
    }
}


mod applications__web_scraper
{
    use futures::stream::{self, StreamExt};
    use reqwest::{Client, Response};
    use scraper::{Html, Selector};
    use std::error::Error;
    use std::time::Instant;

    async fn scrape_url(client: &Client, url: &str) -> Result<Vec<String>, Box<dyn Error>>
    {
        println!("Fetching: {}", url);
        let response: Response = client.get(url).send().await?;
        let body: String = response.text().await?;

        let document: Html = Html::parse_document(&body);
        let selector: Selector = Selector::parse("h2.title")?;

        let titles: Vec<String> = document
            .select(&selector)
            .map(|element| element.text().collect::<String>())
            .collect();

        Ok(titles)
    }

    async fn scrape_websites(urls: Vec<&str>) -> Result<(), Box<dyn Error>>
    {
        let client: Client = Client::new();
        let timestamp: Instant = Instant::now();

        // Sequential approach
        println!("Sequential scraping:");
        for url in &urls {
            let titles: Vec<String> = scrape_url(&client, url).await?;
            println!("Found {} titles on {}", titles.len(), url);
        }
        println!("Sequential scraping took: {:?}", timestamp.elapsed());

        // Concurrent approach
        let timestamp: Instant = Instant::now();
        println!("Concurrent scraping:");
        let results = stream::iter(urls.clone())
            .map(|url| scrape_url(&client, url))
            .buffer_unordered(10) // Process up to 10 requests concurrently
            .collect::<Vec<_>>()
            .await;

        for (i, result) in results.iter().enumerate() {
            match result {
                Ok(titles) => println!("Found {} titles on {}", titles.len(), urls[i]),
                Err(e) => println!("Error scraping {}: {}", urls[i], e),
            }
        }
        println!("Concurrent scraping took: {:?}", timestamp.elapsed());

        Ok(())
    }

    #[tokio::main]
    async fn start_scraping() -> Result<(), Box<dyn Error>>
    {
        let urls = vec![
            "https://news.ycombinator.com",
            "https://reddit.com/r/rust",
            "https://reddit.com/r/programming",
            "https://lobste.rs",
        ];

        scrape_websites(urls).await?;
        Ok(())
    }

    pub fn run() {
        let _ = start_scraping();
    }
}

pub fn future_tests()
{
    // http_request_async::run();

    // example_1::run_multiple_futures_concurrently();
    // example_2::selecting_between_futures();

    applications__web_scraper::run();
}