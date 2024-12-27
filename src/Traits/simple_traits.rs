pub trait Animal {
    fn make_noise(&self);
}

pub struct Dog {}
pub struct Cat {}

impl Animal for Dog {
    fn make_noise(&self) {
        println!("Woof!!")
    }
}

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow!!")
    }
}