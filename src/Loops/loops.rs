use std::collections::HashMap;

fn range_loop()
{
    for i in 0..10 {
        print!("{} ", i)
    }

    for i in (0..10) {
        print!("{} ", i)
    }
    
    // 0 1 2 3 4 5 6 7 8 9 
    // 0 1 2 3 4 5 6 7 8 9 
}

fn infinite_loop()
{
    let mut counter: i32 = 0;
    loop {
        if (counter == 10) {
            break;
        }
        print!("{} ", counter);
        counter += 1;
    }
    
    // 0 1 2 3 4 5 6 7 8 9 
}

fn return_value_from_loop()
{
    let mut counter = 0;
    let result = loop {
        counter += 1;
        print!("{} ", counter);
        if counter == 10 {
            break 1000 + counter ;
        }
    };
    println!(" The result is {}", result);
    
    // 1 2 3 4 5 6 7 8 9 10  The result is 1010
}

fn while_loop()
{
    let mut number = 3;
    while number != 0 {
        print!("{} ", number);
        number -= 1;
    }

    println!("Done");
    // 3 2 1 Done
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

    for idx in (0 .. numbers.len()) {
        print!("{} ", numbers[idx]);
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

fn iterate_reversed()
{
    for number in (0..4).rev() {
        println!("{} ", number);
    }
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
    //  iterate_reversed();

    collection_iterate_array();
    // collection_iterate_map();
}