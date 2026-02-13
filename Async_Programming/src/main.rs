#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod Tokio;
mod Futures;

fn main()
{
    Tokio::tokio_tests();
    // Futures::future_tests();
}
