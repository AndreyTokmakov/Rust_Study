use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T>
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
{
    // NOTE: C++: using Target = T;
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn custom_box()
{
    let stack_var: i32 = 5;
    let my_box: MyBox<i32> = MyBox::new(stack_var);


    assert_eq!(5, stack_var);
    assert_eq!(5, *my_box);
}


pub fn test_all()
{
    custom_box();
}