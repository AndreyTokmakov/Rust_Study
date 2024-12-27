use std::io;

fn read_from_user()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("input text: {}", input);
}


pub fn test_all()
{
    read_from_user();
}
