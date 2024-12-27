use std::mem;

fn slice_info(slice: &[i32])
{
    println!("First slice element: {}", slice[0]);
    println!("Slice size {}", slice.len());
}

fn string_slice()
{
    let str = String::from("0123456789");
    let size: usize = str.len();

    let s1 = &str[1..6];
    println!("{}", s1);

    let s2 = &str[6..size];
    println!("{}", s2);
}

fn array_slices()
{
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];


    println!("First element : {}", xs[0]);
    println!("Second element: {}", xs[1]);
    println!("Array size    : {}", xs.len());
    println!("Array capacity: {} bytes\n", mem::size_of_val(&xs));

    slice_info(&xs);

    println!("\nMake part of the array as slice.\n");
    slice_info(&ys[1 .. 4]);
}

pub fn test_all()
{
    // string_slice();

    array_slices();
}