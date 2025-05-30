use rand::Rng;
use rand::rngs::ThreadRng;

fn test1()
{
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Random numer {}", secret_number);
}


fn test2()
{
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn random_numbers_within_range()
{
    let mut rng: ThreadRng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}


pub fn test_all()
{
    // test1();
    // test2();
    random_numbers_within_range();
}