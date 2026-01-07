use std::path::Iter;

/**
# Rust Vectors
# https://www.compilenrun.com/docs/language/rust/rust-collections/rust-vectors

**/

fn print_vector(vect: &Vec<i32>)
{
    for i in vect {
        print!("{} ", i);
    }
    println!("{}", "");
}

fn iterate()
{
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    for i in &numbers {
        print!("{} ", i);
    }
}

fn iterate_wit_iterator()
{
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    let it= numbers.iter();

    for val in it {
        println!("Got: {}", val);
    }
}

fn iterate_pop()
{
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    while let Some(top) = numbers.pop() {
        print!("{} ", top);
    }
}

fn iterate_and_modify()
{
    let mut numbers = vec![0,1,2,3,4,5,6,7,8,9];
    print_vector(&numbers);

    for i in &mut numbers {
        *i += 10;
    }

    print_vector(&numbers);
}

fn iterate_and_modify__iter_mut()
{
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let numbers_copy: Vec<i32> = numbers.clone();

    for num in numbers.iter_mut() {
        *num += 1; // Mutates each element by adding 1
    }

    println!("{:?} ==> {:?}", numbers_copy, numbers);
    // [1, 2, 3, 4, 5] ==> [2, 3, 4, 5, 6]
}

fn create_test()
{
    {
        let mut numbers: Vec<i32> = Vec::new();

        numbers.push(1);
        numbers.push(2);
        numbers.push(3);

        println!("{:?} of length {}", numbers, numbers.len());
    }

    {
        let numbers: Vec<i32> = vec![1, 2, 3];
        println!("{:?} of length {}", numbers, numbers.len());
    }

    {
        let numbers: Vec<i32> = vec![0; 5];
        println!("{:?} of length {}", numbers, numbers.len());
    }

    {
        let numbers: Vec<i32> = Vec::from([1, 2, 3, 4]);
        println!("{:?} of length {}", numbers, numbers.len());
    }
}

fn access_element()
{
    let numbers: Vec<i32> = vec![0, 1, 2, 3];
    let second: &i32 = &numbers[1];
    println!("The second element of collection {:?} is {}", &numbers, second);
}

fn access_element_non_found()
{
    let values: Vec<i32> = Vec::from([10, 20, 30]);

    let idx: usize = 1;
    if let Some(x) = values.get(idx) {
        println!("Values[{}] = {}", idx, x);
    }
    else {
        println!("Element at {} doesn't exist", idx);
    }


    let idx: usize = 23;
    if let Some(x) = values.get(idx) {
        println!("Values[{}] = {}", idx, x);
    }
    else {
        println!("Element at {} doesn't exist", idx);
    }
}

fn size_and_capacity()
{
    let mut numbers: Vec<i32> = Vec::with_capacity(10);

    for v in 0..3 {
        numbers.push(v);
    }

    println!("{:?} of length {:?} has capacity {}", numbers, numbers.len(), numbers.capacity());
}

fn modify_vector_element()
{
    let mut numbers: Vec<i32> = vec![0, 1, 2, 3];
    numbers[2] = 123;

    println!("{:?}", numbers);
    println!("last element = {:?}",  numbers.last());
}

fn vector_of_enums()
{
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:?} ", i);
    }
}


fn out_of_borders() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

fn  mutable_borrow_error() {
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // TODO: Uncomment me: --> ERROR: ^^^^^^^^^ mutable borrow occurs here
    // v.push(6);

    println!("The first element is: {}", first);
}


fn map__transforming_elements()
{
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();

    println!("Map - Squares: {:?}", squares); // Outputs: [1, 4, 9, 16, 25]
}

fn filter__filtering_elements()
{
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();

    println!("Filter - Evens: {:?}", evens);
    // Outputs: [2, 4]
}


fn fold__reduce_or_accumulate()
{
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);

    println!("Fold - Sum: {}", sum); // Outputs: 15
}

fn experiments()
{
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    let it= numbers.iter();

    for val in it {
        println!("Got: {}", val);
    }
}


pub fn test_all()
{
    // create_test();
    // access_element();
    // access_element_non_found();
    // size_and_capacity();
    // iterate();
    // iterate_wit_iterator();
    // iterate_pop();
    // iterate_and_modify();
    // iterate_and_modify__iter_mut();
    // modify_vector_element();
    // vector_of_enums();
    // out_of_borders();
    // mutable_borrow_error();

    // map__transforming_elements();
    // filter__filtering_elements();
    fold__reduce_or_accumulate();

    // experiments();
}