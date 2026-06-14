#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case,
    unsafe_op_in_unsafe_fn
)]

mod stack_vs_heap_performance;

fn main()
{
    stack_vs_heap_performance::test_all();
}
