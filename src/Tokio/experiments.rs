use std::time::{Instant, SystemTime};
use tokio::time;
use log::Level;
use rwf::prelude::OffsetDateTime;
use tokio::runtime::Runtime;

extern crate chrono;
use chrono::Local;

const DATE_FORMAT: &'static str = "[%Y-%m-%d %H:%M:%S.%3f]";

async fn run()
{
    println!("{} Sleeping", Local::now().format(DATE_FORMAT));
    time::sleep(time::Duration::from_millis(100)).await;
    println!("{} Awake", Local::now().format(DATE_FORMAT));
}

fn run_async_function()
{
    let rt: Runtime = tokio::runtime::Runtime::new().unwrap();
    let fut = run();
    rt.block_on(fut);
}

pub fn test_all()
{
    run_async_function();
}