
fn print_vector(vect: &Vec<i32>)
{
    for i in vect {
        print!("{} ", i);
    }
    println!("{}", "");
}

fn iterate()
{
    {
        let vect: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in &vect {
            print!("{} ", i);
        }
    }

    print!("\n");

    {
        let vect: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in vect {
            print!("{} ", i);
        }
    }
}

fn iterate_and_modify() {
    let mut v = vec![0,1,2,3,4,5,6,7,8,9];

    print_vector(&v);

    for i in &mut v {
        *i += 10;
    }

    print_vector(&v);
}


fn create_test()
{
    {
        let mut vect = Vec::new();

        vect.push(0);
        vect.push(1);
        vect.push(2);
        vect.push(3);

        let third: &i32 = &vect[2];
        println!("The third element is {}", third);
    }

    {
        let vect = vec![1, 2, 3, 4, 5];

        let third: &i32 = &vect[2];
        println!("The third element is {}", third);
    }

    {
        let vec = vec![0; 5];
        println!("{:?}", vec);
    }

    {
        let vec = Vec::from([1, 2, 3, 4]);
        println!("{:?}", vec);
    }
}

fn capacity() {
    let vec: Vec<i32> = Vec::with_capacity(10);

    println!("{:?}", vec);
    println!("vec.len() = {:?}", vec.len());
    println!("vec.capacity() = {:?}", vec.capacity());
}

fn modify_vector_element() {
    {
        let mut v = vec![1, 2, 3, 4, 5];
        v[2] = 123;
        println!("{:?}", v);
    }
}

fn vector_of_enums() {

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
    // create_test();

    iterate();
    // iterate_and_modify();

    // modify_vector_element();

    // vector_of_enums();

    // capacity();

    // out_of_borders();

    // mutable_borrow_error();

    // experiments();
}