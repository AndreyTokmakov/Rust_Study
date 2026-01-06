
mod functions;
mod structs;
mod template_specialisation;
mod bounds_constraints;
mod where_clause;
mod phantom_data;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest_element()
{
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is '{}'", result);

    let char_list = vec!['y', 'm', 'a', 'q' , 'z'];

    let result = largest(&char_list);
    println!("The largest char is '{}'", result);
}

//--------------------------------------------------------------------------------


pub fn test_all()
{
    // functions::test_all();
    // structs::test_all();
    // template_specialisation::test_all();
    // bounds_constraints::test_all();
    // where_clause::test_all();
    phantom_data::test_all();

    // find_largest_element();
    // template_specialization();
}