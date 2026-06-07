#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    non_snake_case
)]

mod borrow_checker_tests
{
    pub fn vector_reallocate_BAD()
    {
        let mut vector: Vec<i32> = vec![1, 2, 3];
        let refFirst: &i32 = &vector[0];        // first element
        vector.push(4);             // may reallocate
        // println!("{}", refFirst);            // r dangles — even though v is still alive
    }

    pub fn vector_reallocate_OK()
    {
        let mut vector: Vec<i32> = vec![1, 2, 3];
        let refFirst: &i32 = &vector[0];     // coordinate to first element
        println!("{}", refFirst);            // ... and its last use
        vector.push(4);         // may reallocate
    }
}

fn main()
{

}
