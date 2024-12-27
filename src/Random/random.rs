use rand::Rng;

fn test1()
{
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Random numer {}", secret_number);
}

pub fn test_all()
{
    test1();
}