use std::fmt::{Debug, Display};


mod function_constraints
{
    use std::fmt::{Debug, Display};

    // Define a function `printer` that takes a generic type `T` which  must implement trait `Display`.
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    trait HasArea
    {
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Rectangle
    {
        length: f64,
        height: f64
    }

    struct Triangle
    {
        length: f64,
        height: f64
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    // The generic `T` must implement `Debug`.
    // Regardless  of the type, this will work properly.
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    // `T` must implement `HasArea`.
    // Any type which meets  the bound can access `HasArea`'s function `area`.
    fn get_area<T: HasArea>(t: &T) -> f64 {
        return t.area()
    }

    pub fn demo()
    {
        let rectangle: Rectangle = Rectangle { length: 3.0, height: 4.0 };

        print_debug(&rectangle);
        println!("Area: {}", get_area(&rectangle));

        let triangle: Triangle = Triangle  { length: 3.0, height: 4.0 };

        // ^ TODO: Try uncommenting these.
        // print_de(&triangle);                   // Error: Does not implement either `HasArea`
        // println!("Area: {}", area(&triangle)); // Error: Does not implement either `Debug`
    }
}

mod check_type_without_input_parameter
{
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red  for Cardinal {}
    impl Blue for BlueJay {}


    // These functions are only valid for types which implement these traits.
    // The fact that the traits are empty is irrelevant.
    fn red<T: Red>(_: &T)  -> &'static str { "red" }

    fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

    pub fn empty_bounds_test()
    {
        let cardinal = Cardinal;
        let blue_jay = BlueJay;
        let turkey   = Turkey;


        println!("A cardinal is {}", red(&cardinal));
        println!("A blue jay is {}", blue(&blue_jay));

        // ^ TODO: Try uncommenting this line.
        // red(&blue_jay);   //  won't work on a blue jay nor vice versa because of the bounds.
        // println!("A turkey is {}", red(&_turkey));
    }
}


mod multiple_bounds
{
    use std::fmt::{Debug, Display};

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }

    pub fn demo()
    {
        let string: &str = "words";
        let array = [1, 2, 3];
        let vec: Vec<i32> = vec![1, 2, 3];

        compare_prints(&string);
        // compare_prints(&array);   // Will not compile - Trait `Display` is not implemented for `[i32; 3]`
        compare_types(&array, &vec);
    }
}



//-------------------------------------------------------------------------------

trait PrintInOption {
    fn print_in_option(self);
}


// Because we would otherwise have to express this as `T: Debug` or 
// use another method of indirect approach, this requires a `where` clause:
impl<T> PrintInOption for T where
    Option<T>: Debug
{
    // We want `Option<T>: Debug` as our bound because that is what's being printed. 
    // Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}


fn using_where_clause()
{
    let vec: Vec<i32> = vec![1, 2, 3];
    vec.print_in_option();
}

pub fn test_all()
{
    // function_constraints::demo();
    // check_type_without_input_parameter::empty_bounds_test();
    // multiple_bounds::demo();
    // using_where_clause();
}