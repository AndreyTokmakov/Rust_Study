
use std::ops::Deref;
use clap::builder::Str;
use crate::operators_overload::dereference;

struct Wrapper<T>(T);

impl<T> Wrapper<T>
{
    fn new(x: T) -> Wrapper<T> {
        Wrapper(x)
    }
}

impl<T> Deref for Wrapper<T>
{
    // NOTE: C++: using Target = T;
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("Wrapper::deref() called ");
        &self.0
    }
}


pub fn test_all()
{
    let wrapped_str: Wrapper<&str> = Wrapper::new("hello");
    // println!("{:?}", *wrapped_str);

    let s: &str = *wrapped_str;
    println!("{:?}", s);
}