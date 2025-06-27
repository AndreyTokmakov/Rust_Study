
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

/** Box
Используется, когда нужно сохранить большое значение в куче или рекурсивные структуры (например, список).
**/

pub fn test_all()
{
    // simple_example();
    pointer_as_Reference();
}