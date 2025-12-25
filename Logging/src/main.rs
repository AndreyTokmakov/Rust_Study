#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

#[path = "Logging/main.rs"] pub mod logging;
#[path = "Tracing/main.rs"] pub mod tracing;



fn main()
{
    logging::test_all();
    // tracing::test_all();
}
