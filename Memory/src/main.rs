#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case,
    unsafe_op_in_unsafe_fn
)]


mod simple_examples;
mod custom_memory_allocation;
mod stack_vs_heap_performance;

fn main()
{
    // simple_examples::test_all();
    // custom_memory_allocation::test_all();
    stack_vs_heap_performance::test_all();
}
