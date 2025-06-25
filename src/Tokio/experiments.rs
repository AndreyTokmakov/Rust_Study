use std::env;
use std::path::PathBuf;
use std::time::{Instant, SystemTime};
use tokio::time;
use log::{log, Level};
use rwf::prelude::OffsetDateTime;
use tokio::runtime::Runtime;
use log::{info, warn, error, debug, LevelFilter};

extern crate chrono;
use chrono::Local;
use rusqlite::fallible_iterator::FallibleIterator;
use simple_logger::SimpleLogger;
use tokio::io::AsyncReadExt;

async fn run()
{
    log::info!("Sleeping");
    time::sleep(time::Duration::from_millis(100)).await;
    log::info!("Awake");
}

fn run_async_function()
{
    let rt: Runtime = tokio::runtime::Runtime::new().unwrap();
    let fut = run();
    rt.block_on(fut);
}

async fn sleeper()
{
    log::info!("Sleeping");
    tokio::time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake");
}

async fn reader()
{
    log::info!("Read some data from file");
    let file_path: &PathBuf = &env::current_dir().unwrap().join("resources/input.txt");
    
    let mut file = tokio::fs::File::open(file_path).await.unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).await.unwrap();
    log::info!("Bytes read: {}", data.len());
}

async fn read_file_async()
{
    tokio::join!(
        sleeper(),
        reader(),
    );
}




pub fn test_all()
{
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let rt: Runtime = tokio::runtime::Runtime::new().unwrap();
    
    // run_async_function();
    let future = read_file_async();
    rt.block_on(future);
}