

fn add(a: i32, b: i32) -> i32 {
    a + b
}


pub fn test_all()
{
    let f: fn(i32, i32) -> i32 = add;
    println!("2 + 3 = {}", f(2, 3));
}