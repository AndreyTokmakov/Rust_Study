
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

pub fn test_all()
{
    // basic_example::demo();
    complex_relationships_between_types::demo();
}