
#[path = "vector.rs"] pub mod vector;
#[path = "hash_map.rs"] pub mod collections_hash_map;
#[path = "arrays.rs"] pub mod collections_arrays;
#[path = "LinkedList.rs"] pub mod LinkedList;

pub fn test_all()
{
    // collections_arrays::test_all();
    vector::test_all();
    // collections_hash_map::test_all();
    // LinkedList::test_all();
}