
#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

#[path = "autotests.rs"] pub mod autotests;


pub fn main()
{
    autotests::test_all();
}