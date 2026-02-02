
mod point_add_operator_overloading_trait
{
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point
    {
        x: i32,
        y: i32
    }

    impl Add for Point
    {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y
            }
        }
    }

    pub fn demo()
    {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3}
        )
    }
}

mod meters_millimeters_type_operator_overloading_trait
{
    use std::ops::Add;

    #[derive(Debug)]
    struct Meters(u32);

    #[derive(Debug)]
    struct Millimeters(u32);

    impl Add<Meters> for Millimeters
    {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }

    pub fn demo()
    {
        let mm: Millimeters = Millimeters(750);
        let m: Meters = Meters(1);

        let res: Millimeters = mm + m;
        println!("{:?}", res);

        // Millimeters(1750)
    }
}

mod add_int_32_example
{
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

    pub fn demo()
    {
        let var1: Int32One = Int32One { value: 1 };
        let var2: Int32Two = Int32Two { value: 1 };

        let resultOne = var2 + var1;
        let resultTwo = var1 + var2;

        println!("{:?}", resultOne);
        println!("{:?}", resultTwo);
    }
}



pub fn test_all()
{
    // add_int_32_example::demo();
    // point_add_operator_overloading_trait::demo();
    meters_millimeters_type_operator_overloading_trait::demo();
}