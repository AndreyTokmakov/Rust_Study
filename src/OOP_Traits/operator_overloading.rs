
use std::ops;

#[derive(Debug, Copy, Clone)]
struct Int32One {
    value: i32,
}

#[derive(Debug, Copy, Clone)]
struct Int32Two {
    value: i32,
}


impl ops::Add<Int32Two> for Int32One {
    type Output = Int32Two;

    fn add(self, _rhs: Int32Two) -> Int32Two {
        println!("> Int32One.add(Bar) was called");
        Int32Two { value: self.value + _rhs.value }
    }
}

impl ops::Add<Int32One> for Int32Two {
    type Output = Int32One;

    fn add(self, _rhs: Int32One) -> Int32One {
        println!("> Int32One.add() was called");
        Int32One { value: self.value + _rhs.value }
    }
}


pub fn test_all()
{
    let var1: Int32One = Int32One { value: 1 };
    let var2: Int32Two = Int32Two { value: 1 };

    let resultOne = var2 + var1;
    let resultTwo = var1 + var2;

    println!("{:?}", resultOne);
    println!("{:?}", resultTwo);
}