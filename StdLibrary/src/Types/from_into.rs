
use std::convert::From;

#[derive(Debug)]
struct Number
{
    value: i32,
}

impl From<i32> for Number
{
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn From()
{
    let value: i32 = 5;
    let num: Number = Number::from(value);
    println!("My number: {:?}", num);
}

fn Into()
{
    let value: i32 = 5;
    let num: Number = value.into();
    println!("My number: {:?}", num);
}

pub fn test_all()
{
    From();
    Into();
}