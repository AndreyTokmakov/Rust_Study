
mod custom_smart_pointer;
mod Rc;
mod drop;
mod Box;
mod Arc;
mod Cell;
mod RefCell;
mod Mutex;

mod pass_String_as_str
{
    fn is_strong<T: AsRef<str>>(password: T) -> bool {
        password.as_ref() .len() > 5
    }

    pub fn test()
    {
        let str_password: &str = "justok";
        let string_pass: String = "qwerty".to_string();

        let is_strong_1: bool = is_strong(str_password);
        let is_strong_2: bool = is_strong(string_pass);
    }
}

/**
Smart Pointers in Rust

| Name       | Ownership   | Mutability    | Thread-safe    | Use case                                        |
|------------|-------------|---------------|----------------|-------------------------------------------------|
| Box<T>     | Single      | Yes (if T)    | Yes            | Heap allocation, recursive types, trait objects |
| Rc<T>      | Multiple    | No            | No             | Single-threaded shared read-only access         |
| Arc<T>     | Multiple    | No            | Yes            | Multi-threaded shared read-only access          |
| RefCell<T> | Single      | Yes (runtime) | No             | Interior mutability, borrow checking at runtime |
| Cell<T>    | Single      | Yes (copy)    | No             | For Copy types, no references involved          |
| Weak<T>    | Weak (none) | No            | Same as Rc/Arc | Break reference cycles                          |

Common Combinations

| Combination                | Thread-safe | Use case                                       |
|----------------------------|-------------|------------------------------------------------|
| Rc<RefCell<T>>             | No          | Multiple owners, mutable data in one thread    |
| Arc<Mutex<T>>              | Yes         | Multiple owners, mutable data across threads   |
| Arc<RwLock<T>>             | Yes         | Multiple readers, single writer across threads |
| Box<dyn Trait>             | Yes         | Trait objects, dynamic dispatch                |
**/

// https://www.compilenrun.com/docs/language/rust/rust-memory-management/


pub fn test_all() 
{
    // pass_String_as_str::test();
    // custom_smart_pointer::test_all();
    // Box::test_all();
    // drop::test_all(); 
    Rc::test_all();
    // Arc::test_all();
    // Cell::test_all();
    // RefCell::test_all();
    // Mutex::test_all();
}
