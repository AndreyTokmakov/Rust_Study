use rand::Rng;
use rand::rngs::ThreadRng;
// use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn test1()
{
    let secret_number = rand::rng().random_range(1..101);
    println!("Random numer {}", secret_number);
}


fn test2()
{
    let mut rng = rand::rng();

    let n1: u8 = rng.random();
    let n2: u16 = rng.random();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.random::<u32>());
    println!("Random i32: {}", rng.random::<i32>());
    println!("Random float: {}", rng.random::<f64>());
}

fn random_numbers_within_range()
{
    let mut rng: ThreadRng = rand::rng();
    println!("Integer: {}", rng.random_range(0 .. 10));
    println!("Float: {}", rng.random_range(0.0 .. 10.0));
}

/*
impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}
*/

pub fn test_all()
{
    // test1();
    test2();
    // random_numbers_within_range();
}

// INFO: https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html