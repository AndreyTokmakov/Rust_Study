#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

#[path = "command/command_1.rs"] pub mod command_1;
#[path = "command/command_func_ptr.rs"] pub mod command_func_ptr;
#[path = "command/command_fn_traits.rs"] pub mod command_fn_traits;

mod State;
mod TaskPool;

fn main()
{
    // State::test_all();
    // TaskPool::test_all();

    // command_1::test_all();
    // command_func_ptr::test_all();
    command_fn_traits::test_all();
}
