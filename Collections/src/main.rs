#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

#[path = "vector.rs"] pub mod vector;
#[path = "hash_map.rs"] pub mod hash_map;
#[path = "btree_map.rs"] pub mod btree_map;
#[path = "hash_set.rs"] pub mod hash_set;
#[path = "arrays.rs"] pub mod collections_arrays;
#[path = "LinkedList.rs"] pub mod LinkedList;

fn main()
{
    // collections_arrays::test_all();
    vector::test_all();
    // hash_map::test_all();
    // btree_map::test_all();
    // hash_set::test_all();
    // LinkedList::test_all();
}