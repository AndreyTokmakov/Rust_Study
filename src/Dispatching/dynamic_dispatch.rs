
trait Animal
{
    fn make_noise(&self);
}

struct Dog {}
struct Cat {}

impl Animal for Dog
{
    fn make_noise(&self)
    {
        println!("Woof!!")
    }
}

impl Animal for Cat
{
    fn make_noise(&self)
    {
        println!("Meow!!")
    }
}

fn test()
{
    let vect: Vec<Box<dyn Animal>> = vec![Box::new(Dog{}), Box::new(Cat{})];
    for item in &vect {
        item.make_noise();
    }
}

pub fn test_all()
{
    test();
}