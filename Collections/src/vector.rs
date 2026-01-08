use std::path::Iter;

/**
# Rust Vectors
# https://www.compilenrun.com/docs/language/rust/rust-collections/rust-vectors

**/

mod basics
{
    pub fn print_vector(vector: &Vec<i32>)
    {
        for i in vector {
            print!("{} ", i);
        }
        println!("{}", "");
    }

    pub fn push_back()
    {
        let mut numbers: Vec<i32> = vec![1,2,3,];

        numbers.push(4);
        numbers.push(5);
        numbers.push(6);
        println!("{:?}", &numbers);
    }

    pub fn extend_vector()
    {
        let mut numbers: Vec<i32> = vec![1,2,3,];
        let more_numbers: Vec<i32> = vec![4,5,6];

        numbers.extend(more_numbers);
        println!("{:?}", &numbers);
    }

    pub fn pop_elements()
    {
        let mut numbers: Vec<i32> = vec![1,2,3,4,5];
        print!("{:?} ==> ", &numbers);

        numbers.pop();
        println!("{:?}", &numbers);

        // [1, 2, 3, 4, 5] ==> [1, 2, 3, 4]
    }

    pub fn remove_elements()
    {
        let mut numbers: Vec<i32> = vec![1,2,3,4,5];
        print!("{:?} ==> ", &numbers);

        numbers.remove(3); // Remove element with IDX = 3
        println!("{:?}", &numbers);

        // [1, 2, 3, 4, 5] ==> [1, 2, 3, 5]
    }

    pub fn iterate()
    {
        let numbers: Vec<i32> = vec![1,2,3,4,5];
        for i in &numbers {
            print!("{} ", i);
        }
    }

    pub fn iterate_enumerate()
    {
        let numbers: Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
        for (index, element) in numbers.iter().enumerate() {
            println!("Index: {}, Value: {}", index, element);
        }

        // Index: 0, Value: 1
        // .....
        // Index: 8, Value: 9
    }

    pub fn iterate_wit_iterator()
    {
        let numbers: Vec<i32> = vec![1,2,3,4,5];
        let it= numbers.iter();

        for val in it {
            println!("Got: {}", val);
        }
    }

    pub fn iterate_pop()
    {
        let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        while let Some(top) = numbers.pop() {
            print!("{} ", top);
        }
    }

    pub fn iterate_and_modify()
    {
        let mut numbers = vec![0,1,2,3,4,5,6,7,8,9];
        print_vector(&numbers);

        for i in &mut numbers {
            *i += 10;
        }

        print_vector(&numbers);
    }

    pub fn iterate_and_modify__iter_mut()
    {
        let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        let numbers_copy: Vec<i32> = numbers.clone();

        for num in numbers.iter_mut() {
            *num += 1; // Mutates each element by adding 1
        }

        println!("{:?} ==> {:?}", numbers_copy, numbers);
        // [1, 2, 3, 4, 5] ==> [2, 3, 4, 5, 6]
    }

    pub fn create_test()
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

    pub fn access_element()
    {
        let numbers: Vec<i32> = vec![0, 1, 2, 3];
        let second: &i32 = &numbers[1];
        println!("The second element of collection {:?} is {}", &numbers, second);
    }

    pub fn access_element_non_found()
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

    pub fn get_element_check_exising()
    {
        let numbers: Vec<i32> = vec![0, 1, 2, 3, 4, 5];

        let check_element_by_index = |values: &Vec<i32>, idx: usize| {
            match numbers.get(idx) {
                Some(element) => println!("Element at index {}: {}", idx, element),
                None => println!("Element at index {} doesn't exist", idx),
            }
        };

        check_element_by_index(&numbers, 1);
        check_element_by_index(&numbers, 5);
    }

    pub fn size_and_capacity()
    {
        let mut numbers: Vec<i32> = Vec::with_capacity(10);

        for v in 0..3 {
            numbers.push(v);
        }

        println!("{:?} of length {:?} has capacity {}", numbers, numbers.len(), numbers.capacity());
    }

    pub fn modify_vector_element()
    {
        let mut numbers: Vec<i32> = vec![0, 1, 2, 3];
        numbers[2] = 123;

        println!("{:?}", numbers);
        println!("last element = {:?}",  numbers.last());
    }

    pub fn vector_of_enums()
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

    pub fn out_of_borders()
    {
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    }

    pub fn  mutable_borrow_error()
    {
        let v = vec![1, 2, 3, 4, 5];
        let first = &v[0];

        // TODO: Uncomment me: --> ERROR: ^^^^^^^^^ mutable borrow occurs here
        // v.push(6);

        println!("The first element is: {}", first);
    }


    pub fn map__transforming_elements()
    {
        let numbers: Vec<i32> = vec![1,2,3,4,5];
        let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();

        println!("Map - Squares: {:?}", squares); // Outputs: [1, 4, 9, 16, 25]
    }

    pub fn filter__filtering_elements()
    {
        let numbers: Vec<i32> = vec![1,2,3,4,5];
        let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();

        println!("Filter - Evens: {:?}", evens);
        // Outputs: [2, 4]
    }

    pub fn fold__reduce_or_accumulate()
    {
        let numbers: Vec<i32> = vec![1,2,3,4,5];
        let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);

        println!("Fold - Sum: {}", sum); // Outputs: 15
    }
}

mod practical_applications
{
    use std::io;

    pub fn user_input_collection()
    {
        let mut input: String = String::new();
        let mut numbers: Vec<i32> = Vec::new();

        println!("Enter numbers, one per line (enter 'done' when finished):");
        loop {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            let input: &str = input.trim();
            if input == "q" || input == "quit" {
                break;
            }

            match input.parse::<i32>() {
                Ok(number) => {
                    numbers.push(number);
                    println!("Added: {}. Current list: {:?}", number, numbers);
                },
                Err(_) => println!("That's not a valid number, try again."),
            }
        }

        if !numbers.is_empty() {
            let sum: i32 = numbers.iter().sum();
            let average: f64 = sum as f64 / numbers.len() as f64;

            println!("Statistics:\n\tCount: {}\n\tSum: {}\n\tAverage: {:.2}",
                     numbers.len(), sum, average);
        } else {
            println!("No numbers were entered.");
        }
    }

    pub fn filtering_and_transforming_data()
    {
        // Initial data: product prices in dollars
        let prices: Vec<f64> = vec![29.99, 19.95, 8.99, 110.0, 65.5, 42.0, 17.25, 88.0];
        println!("Original prices: {:?}", prices);

        // Task 1: Find all products under $20
        let affordable = prices.iter()
            .filter(|&&price| price < 20.0)
            .collect::<Vec<_>>();
        println!("Affordable products (under $20): {:?}", affordable);

        // Task 2: Apply 10% discount to all prices
        let discounted: Vec<f64> = prices.iter()
            .map(|&price| price * 0.9)
            .collect();
        println!("Prices after 10% discount: {:?}", discounted);

        // Task 3: Find expensive products and apply a larger discount
        let special_offer: Vec<f64> = prices.iter()
            .map(|&price| if price > 50.0 {
                price * 0.8 // 20% off for expensive items
            } else {
                price * 0.9 // 10% off for regular items
            })
            .collect();
        println!("Special offer prices: {:?}", special_offer);
    }
}


mod experiments
{
    pub fn test()
    {
        let numbers: Vec<i32> = vec![1,2,3,4,5];
        let it= numbers.iter();

        for val in it {
            println!("Got: {}", val);
        }
    }
}

pub fn test_all()
{
    // basics::create_test();

    // basics::push_back();
    // basics::extend_vector();

    // basics::pop_elements();
    // basics::remove_elements();

    // basics::access_element();
    // basics::access_element_non_found();
    // basics::get_element_check_exising();

    // basics::size_and_capacity();

    // basics::iterate();
    // basics::iterate_enumerate();
    // basics::iterate_wit_iterator();
    // basics::iterate_pop();
    // basics::iterate_and_modify();
    // basics::iterate_and_modify__iter_mut();

    // basics::modify_vector_element();

    // basics::vector_of_enums();
    // basics::out_of_borders();
    // basics::mutable_borrow_error();

    // basics::map__transforming_elements();
    // basics::filter__filtering_elements();
    // basics::fold__reduce_or_accumulate();

    // practical_applications::user_input_collection();
    practical_applications::filtering_and_transforming_data();

    // experiments::test();
}