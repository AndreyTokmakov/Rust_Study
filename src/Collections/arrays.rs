
use std::mem;

fn array_info()
{
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];


    println!("First element : {}", xs[0]);
    println!("Second element: {}", xs[1]);
    println!("Array size    : {}", xs.len());
    println!("Array capacity: {} bytes", mem::size_of_val(&xs));

}

fn init_with_zeros()
{
    let array: [i32; 3] = [0; 3];

    for i in array {
        println!("{}", i);
    }
}

fn stack_array() {
    // A stack-allocated array
    let array: [i32; 3] = [1, 2, 3];

    for i in array {
        println!("{}", i);
    }
}

fn iterate_array() {
    let array: [i32; 3] = [1, 2, 3];

    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{i}] = {x}");
    }

    println!("-------------------------------------------------------------------");

    // You can explicitly iterate an array by value using `IntoIterator::into_iter`
    for item in IntoIterator::into_iter(array).enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{i}] = {x}");
    }

}


fn tests()
{
    let array: [i32; 3] = [0; 3];

    for i in array {
        println!("{}", i);
    }
}

pub fn test_all()
{
    // init_with_zeros();
    // stack_array();

    // iterate_array();
    // tests();

    array_info();
}