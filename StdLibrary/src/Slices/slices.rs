use std::mem;

fn slice_info(slice: &[i32])
{
    println!("First slice element: {}", slice[0]);
    println!("Slice size {}", slice.len());
}

fn take_all_full_slice()
{
    let mut array: [i32; 3] = [1, 2, 3];
    let slice: &mut[i32] = &mut array[..];

    assert_eq!(slice, &[1, 2, 3]);
    assert_eq!(3, slice.len());
}

fn mutable_slice()
{
    let mut array: [i32; 3] = [1, 2, 3];
    let slice: &mut[i32] = &mut array[..];
    slice[1] = 7;

    assert_eq!(&array, &[1, 7, 3]);
}

fn array_to_splice_inplace()
{
    let slice: &[&str] = &["one", "two", "three"];

    println!("{:?}", &slice);
    assert_eq!(slice.len(), 3);
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


fn create_splice()
{
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7 , 8, 9, 10];

    let empty1: &[i32] = &array[0..0];
    let empty2: &[i32] = &array[..0];    // same as &x[0..0]
    let empty3: &[i32] = &array[1..1];   // empty subslice in the middle
    let empty4: &[i32] = &array[3..3];   // subslice after the last element
    let empty5: &[i32] = &array[3..];    // same as &x[3..3]

    let full1: &[i32] = &array[..];
    let full2: &[i32] = &array[0..array.len()];
    let full3: &[i32] = &array[0..];
    assert_eq!(full1, full2);
    assert_eq!(full1, full3);

    let part1: &[i32] = &array[..5];
    println!("part1: {:?}", part1);
}

pub fn test_all()
{
    create_splice();
    // take_all_full_slice();
    // mutable_slice();
    // array_to_splice_inplace();
    // string_slice();
    // array_slices();
}