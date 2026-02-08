
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



pub fn test_all() 
{
    // pass_String_as_str::test();
    // custom_smart_pointer::test_all();
    // Box::test_all();
    // drop::test_all();
    // Rc::test_all();
    Arc::test_all();
    // Cell::test_all();
    // RefCell::test_all();
    // Mutex::test_all();
}
