use std::collections::HashMap;


mod for_loops
{
    pub fn range_loop()
    {
        for i in 0..10 {
            print!("{} ", i)
        }

        for i in (0..10) {
            print!("{} ", i)
        }

        for i in (0..=10) {
            print!("{} ", i)
        }

        // 0 1 2 3 4 5 6 7 8 9 
        // 0 1 2 3 4 5 6 7 8 9
        // 0 1 2 3 4 5 6 7 8 9 10 
    }

    pub fn iter_loop()
    {
        let fruits = ["apple", "banana", "cherry"];
        for fruit in fruits.iter() {
            println!("{}", fruit);
        }
    }
    
    pub fn enumerate_loop()
    {
        let items = ["one", "two", "three"];
        for (index, item) in items.iter().enumerate() {
            println!("Item {} is {}", index, item);
        }
        
        // Item 0 is one
        // Item 1 is two
        // Item 2 is three
    }
}


mod while_loops
{
    pub fn basic()
    {
        let mut number = 3;
        while (number != 0) {
            print!("{} ", number);
            number -= 1;
        }

        println!("Done");
        // 3 2 1 Done
    }
    
    
    pub fn while_let()
    {
        let mut stack = vec![1, 2, 3];

        // while let example (idiomatic for popping collections)
        while let Some(top) = stack.pop() {
            println!("Popped: {}", top);
        }
        
        println!("Stack size is {}", stack.len());
    }
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



//      loop	    loop { ... break; }
//      while	    while condition { ... }
//      for in	    for x in 0..10 { ... }
//      while let	while let Some(x) = opt { ... }

pub fn test_all()
{
    // for_loops::range_loop();
    // for_loops::iter_loop();
    // for_loops::enumerate_loop();

    while_loops::basic();
    while_loops::while_let();

    // infinite_loop();
    // return_value_from_loop();
    // loop_labels();
    // iterate_reversed();

    // collection_iterate_array();
    // collection_iterate_map();
}