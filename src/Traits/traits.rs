
pub mod simple_traits;

use simple_traits::Animal;
use simple_traits::Dog;
use simple_traits::Cat;

pub fn test_all()
{
    let a1: Box<dyn Animal> = Box::new(Dog{});
    let a2: Box<dyn Animal> = Box::new(Cat{});

    // let vect: Vec<Box<dyn Animal>> = vec![Box::new(Dog{}), Box::new(Cat{})];
}