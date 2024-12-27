use std::cmp::Ordering;

pub fn test_all()
{

    let val = 434;
    let number = 123;

    match val.cmp(&number) {
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("YEqual"),
    }
}