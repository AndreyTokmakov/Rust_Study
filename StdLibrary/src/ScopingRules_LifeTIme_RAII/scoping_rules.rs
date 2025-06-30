
mod RAII;
mod ownership_moves;
mod mutability;
mod partial_moves;
mod aliasing;
mod ref_usage_examples;

pub fn test_all()
{
    // RAII::test_all();
    //ownership_moves::test_all();
    // mutability::test_all();
    // partial_moves::test_all();
    // aliasing::test_all();
    ref_usage_examples::test_all();
}