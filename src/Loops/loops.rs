
fn range_loop()
{
    for i in 0..10 {
        print!("{} ", i)
    }
}

fn infinite_loop()
{
    let mut counter: i32 = 0;
    loop {
        if (counter == 10) {
            break;
        }
        println!("{}", counter);
        counter += 1;
    }
}

fn return_value_from_loop()
{
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break 1000 + counter ;
        }
    };
    println!("The result is {}", result);
}

fn while_loop()
{
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Done");
}

fn collection_iterate_1() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

fn collection_iterate_2()
{
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn iterate_range()
{
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Done");
}


pub fn test_all()
{
    range_loop();

    // infinite_loop();
    // return_value_from_loop();
    // while_loop();

    // collection_iterate_1();
    // collection_iterate_2();

    // iterate_range();
}