
mod custom_smart_pointer;
mod reference_counter;
mod drop;


fn simple_example()
{
    let boxed_int: Box<i32> = Box::new(5);
    println!("boxed_int = {boxed_int}");
}


fn pointer_as_Reference()
{
    let stack_var: i32 = 5;
    let ref_var: &i32 = &stack_var;
    let ptr: Box<i32> = Box::new(stack_var);

    assert_eq!(5, stack_var);
    assert_eq!(5, *ref_var);
    assert_eq!(5, *ptr);
}

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
    pass_String_as_str::test();
    
    // simple_example();
    // pointer_as_Reference();

    // custom_smart_pointer::test_all();
    // drop::test_all();
    // reference_counter::test_all();
}
