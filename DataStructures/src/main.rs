#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

#[path = "Cache/main.rs"] pub mod cache;

fn main()
{
    cache::test_all()
}
