#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod threads;
mod mutex;
mod arc;
mod atomics;
mod locking;
mod sender_receiver;
mod crossbeam_channel;
mod mpsc_channel;
mod mpsc_channel_tokio;
mod mpmc_channel;
mod flume_channel;
mod condition_variables;
mod barrier;
mod thread_local;

pub fn main()
{
    // threads::test_all();
    // atomics::test_all();
    // barrier::test_all();
    thread_local::test_all();
    // condition_variables::test_all();
    // mutex::test_all();
    // arc::test_all();
    // locking::test_all();
    // sender_receiver::test_all();
    // mpsc_channel::test_all(); 
    // mpsc_channel_tokio::test_all();
    // mpmc_channel::test_all();
    // crossbeam_channel::test_all();
    // flume_channel::test_all();
}