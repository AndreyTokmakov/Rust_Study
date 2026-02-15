#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    non_snake_case
)]


#[path = "tokio/Tokio.rs"] pub mod Tokio;
#[path = "std/Futures.rs"] pub mod Futures;
#[path = "executors/CustomExecutor.rs"] pub mod CustomExecutor;


fn main()
{
    // Tokio::tokio_tests();
    // Futures::future_tests();
    CustomExecutor::testAll();
}
