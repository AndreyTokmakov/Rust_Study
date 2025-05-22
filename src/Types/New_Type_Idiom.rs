

#[derive(Debug)]
struct Years(i64);

fn obtain_value()
{
    // To obtain the newtype's value as the base type, you may use the tuple or destructuring syntax like so:
    
    let years: Years = Years(42);
    let years_as_primitive_1: i64 = years.0;      // Tuple
    let Years(years_as_primitive_2) = years; // Destructuring

    println!("years: {:?}", years);
    println!("years_as_primitive_1: {:?}", years_as_primitive_1);
    println!("years_as_primitive_2: {:?}", years_as_primitive_2);
}


pub fn test_all()
{
    obtain_value();
}