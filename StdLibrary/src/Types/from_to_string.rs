
pub fn test_all()
{
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse::<i32>().unwrap();

    let sum: i32 = parsed + turbo_parsed;
    println!("Сумма: {:?}", sum);
}