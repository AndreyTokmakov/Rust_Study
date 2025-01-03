use std::collections::HashMap;

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

fn collection_iterate_array()
{
    let numbers = [10, 20, 30, 40, 50];

    for element in numbers {
        print!("{} ", element);
    }
    println!();

    let mut index = 0;
    while index < 5 {
        print!("{} ", numbers[index]);
        index += 1;
    }
    println!();

    for element in (1..5) {
        print!("{element} ");
    }
    println!();
}

fn collection_iterate_map()
{
    let map: HashMap<&str, i32> = HashMap::from([
        ("I", 1), ("II", 2), ("III", 3), ("IV", 4), ("V", 5),
    ]);

    for (key, value) in map.iter() {
        println!("{key} = {value} ");
    }
}

fn iterate_range()
{
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Done");
}


fn loop_labels()
{
    let mut count = 0;
    'loop_one: loop {
        println!("count (loop_one): {count}");
        let mut remaining = 10;

        loop {
            println!("remaining (inner loop): {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'loop_one;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


pub fn test_all()
{
    // range_loop();

    // infinite_loop();
    // return_value_from_loop();
    // while_loop();
    // loop_labels();

    // collection_iterate_array();
    collection_iterate_map();

    // iterate_range();
}