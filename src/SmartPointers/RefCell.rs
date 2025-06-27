
mod basic
{
    use std::cell::RefCell;
    pub fn example()
    {
        let ptr: RefCell<i32> = RefCell::new(5);

        *ptr.borrow_mut() += 1;
        println!("x = {}", ptr.borrow()); // x = 6
    }
}


/**
    RefCell<T> — мутация при immut borrow
    Allows mutation even with an immutable reference.
    ❗ Check for multiple mutations - at runtime, not at compile time.
**/
pub fn test_all()
{
    basic::example();

}