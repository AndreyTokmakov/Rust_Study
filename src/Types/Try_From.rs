
pub fn test_all() {
    test_greater_than_zero();
    // test_even_number_cast();
}


struct GreaterThanZero(i32);

impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            println!("---1----");
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            println!("---1----");
            Ok(GreaterThanZero(value))
        }
    }
}

fn test_greater_than_zero() {
    let big_number = 1_000_000_000_000i64;
    let small_number = 1_000;

    // Silently truncates `big_number`, requires detecting
    // and handling the truncation after the fact.
    let smaller_number = big_number as i32;

    println!("{}\n", smaller_number);

    // Returns an error because `big_number` is too big to fit in an `i32`.
    let try_smaller_number = i32::try_from(big_number);
    if try_smaller_number.is_err() {
        println!("Failed to cast 'big_number'\n");
    } else {
        println!("OK")
    }

    // Returns `Ok(3)`.
    let try_successful_small_number = i32::try_from(small_number);
    if try_successful_small_number.is_err() {
        println!("Failed to cast 'small_number'\n");
    } else {
        println!("OK")
    }
}

//--------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            println!("Parsed OK");
            Ok(EvenNumber(value))
        } else {
            println!("Failed to parse");
            Err(())
        }
    }
}

fn test_even_number_cast() {
    let try_even = i32::try_from(2);
    println!("{}", try_even.unwrap());

    let try_odd = i32::try_from(3);
    println!("{}", try_odd.unwrap());
}