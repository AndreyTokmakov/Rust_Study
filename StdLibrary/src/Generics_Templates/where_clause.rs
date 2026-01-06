
/**

# Rust Where Clauses
# https://www.compilenrun.com/docs/language/rust/rust-generics/rust-where-clauses/

=====================================================================================

# The basic syntax for a where clause is:
fn function_name<T1, T2, ...>(parameters) -> return_type
    where T1: Trait1 + Trait2,
          T2: Trait3 + Trait4,
          // More constraints...
{
    // Function body
}

# For structs and enums:
struct StructName<T1, T2, ...>
    where T1: Trait1 + Trait2,
          T2: Trait3 + Trait4,
          // More constraints...
{
    // Struct fields
}

=====================================================================================

fn print_pair<T, U>(first: T, second: U)
    where T: Display,
             U: Display,
{
    println!("Pair: {} and {}", first, second);
}

fn main() {
    print_pair(42, "hello");
    print_pair(3.14, true);
}

**/

mod basic_example
{
    use std::fmt;
    use std::fmt::{Debug, Display};

    #[derive(Debug)]
    struct Request
    {
        data: String
    }

    #[derive(PartialEq)]
    struct Response
    {
        data: String
    }

    impl Display for Response
    {
        fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
            write!(format, "Response [ data = {}]",self.data)
        }
    }

    fn process_data_1<T: Clone + Debug,U: Display + PartialEq>(data: T, output: U)
    {
        println!("Process data: {:?} ==> Output: {}", data, output);
    }

    fn process_data_with_where<T, U>(data: T, output: U)
        where T: Clone + Debug,
              U: Display + PartialEq,
    {
        println!("Process data: {:?} ==> Output: {}", data, output);
    }

    pub fn demo()
    {
        let request: Request = Request { data: "Payload".to_string() };
        let response: Response = Response { data: "OutPut".to_string() };

        process_data_1(&request, &response);
        process_data_with_where(&request, &response);
    }
}

mod complex_relationships_between_types
{
    use std::hash::Hash;
    use std::collections::HashMap;

    fn count_occurrences<T, C>(collection: C) -> HashMap<T, usize>
        where T: Eq + Hash + Clone,
              C: IntoIterator<Item=T>,
    {
        let mut counts: HashMap<T, usize> = HashMap::new();
        for item in collection {
            *counts.entry(item).or_insert(0) += 1;
        }

        counts
    }

    pub fn demo()
    {
        let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
        let word_counts = count_occurrences(words);

        for (word, count) in word_counts {
            println!("{}: {}", word, count);
        }
    }
}

mod generic_data_processor
{
    use std::fmt::{Debug, Display};
    use std::str::FromStr;
    use std::error::Error;

    // A generic data processor that can convert between different types
    struct DataProcessor<T, U>
        where T: Debug + Clone,
              U: Display + FromStr,
              <U as FromStr>::Err: Error,
    {
        source_data: Vec<T>,
        phantom: std::marker::PhantomData<U>,
    }

    impl<T, U> DataProcessor<T, U>
        where T: Debug + Clone + Display,
              U: Display + FromStr,
              <U as FromStr>::Err: Error,
    {
        fn new(source_data: Vec<T>) -> Self {
            DataProcessor {
                source_data,
                phantom: std::marker::PhantomData,
            }
        }

        fn process(&self) -> Vec<Result<U, Box<dyn Error>>> where <U as FromStr>::Err: 'static {
            self.source_data.iter().map(|item| {
                    // Convert T to string
                    let item_str: String = item.to_string();
                    // Try to parse the string to U
                    let result = U::from_str(&item_str)
                        .map_err(|e| Box::new(e) as Box<dyn Error>);

                    result
                }).collect()
        }

        fn display_all(&self) {
            println!("Source data:");
            for item in &self.source_data {
                println!("  {:?}", item);
            }
        }
    }

    pub fn demo()
    {
        // Process integers to floats
        let int_processor: DataProcessor<i32, f64> = DataProcessor::<i32, f64>::new(vec![1, 2, 3, 4, 5]);
        int_processor.display_all();

        let results = int_processor.process();
        println!("Processed results:");
        for result in results {
            match result {
                Ok(val) => println!("  Success: {}", val),
                Err(e) => println!("  Error: {}", e),
            }
        }

        // Process strings to integers with potential errors
        let string_processor: DataProcessor<String, i32>  = DataProcessor::<String, i32>::new(
            vec![
                "123".to_string(),
                "456".to_string(),
                "not_a_number".to_string(),
                "789".to_string(),
            ]
        );
        string_processor.display_all();

        let results = string_processor.process();
        println!("Processed results:");
        for result in results {
            match result {
                Ok(val) => println!("  Success: {}", val),
                Err(e) => println!("  Error: {}", e),
            }
        }
    }

    // Source data:
    //   1
    //   2
    //   3
    //   4
    //   5
    // Processed results:
    //   Success: 1
    //   Success: 2
    //   Success: 3
    //   Success: 4
    //   Success: 5
    
    // Source data:
    //   "123"
    //   "456"
    //   "not_a_number"
    //   "789"
    // Processed results:
    //   Success: 123
    //   Success: 456
    //   Error: invalid digit found in string
    //   Success: 789
}

pub fn test_all()
{
    // basic_example::demo();
    // complex_relationships_between_types::demo();
    generic_data_processor::demo();
}