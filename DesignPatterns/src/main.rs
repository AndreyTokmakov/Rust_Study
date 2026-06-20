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
#[path = "factory/Factory.rs"] pub mod factory;
#[path = "builder/Builder.rs"] pub mod builder;
#[path = "strategy/Strategy.rs"] pub mod strategy;
#[path = "state/State.rs"] pub mod state;
#[path = "undo/main.rs"] pub mod undo;
#[path = "observer/Observer.rs"] pub mod observer;

mod TaskPool;

fn main()
{
    // state::test_all();
    // TaskPool::test_all();

    // command_1::test_all();
    // command_func_ptr::test_all();
    // command_fn_traits::test_all();

    // factory::test_all();
    // builder::test_all();
    // strategy::test_all();

    observer::test_all();

    // undo::test_all();
}
