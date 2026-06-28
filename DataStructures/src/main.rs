#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

#[path = "Cache/main.rs"] pub mod cache;
#[path = "ObservableWrapper/ObservableWrapper.rs"] pub mod ObservableWrapper;

fn main()
{
    // cache::test_all()
    ObservableWrapper::observer_wrapper::test_all()
}
