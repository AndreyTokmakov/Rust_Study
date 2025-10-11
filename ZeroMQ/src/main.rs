#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod clients;
mod experiments;

fn main()
{
    clients::test_all();
    // experiments::test_all();
}
