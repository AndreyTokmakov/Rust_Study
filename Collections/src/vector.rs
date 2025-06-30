
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


fn experiments()
{
    let mut v = vec![1, 2, 3, 4, 5];

    /*
    let mut third = &mut v[2];
    println!("The third element is {}", third);

    third = 65;

    println!("The third element is {}", third);*/

    v[2] = 123;


    println!("{:?}", v);

    /*
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }*/
}


pub fn test_all()
{
    create_test();
    // access_element();

    // size_and_capacity();

    // iterate();
    // iterate_pop();
    // iterate_and_modify();

    // modify_vector_element();

    // vector_of_enums();

    // out_of_borders();

    // mutable_borrow_error();

    // experiments();
}