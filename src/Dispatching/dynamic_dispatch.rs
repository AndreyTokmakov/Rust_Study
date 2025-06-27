

mod example_simple
{
    struct Dog {}
    struct Cat {}

    trait Animal {
        fn make_noise(&self);
    }
    
    impl Animal for Dog {
        fn make_noise(&self)
        {
            println!("Woof!!")
        }
    }

    impl Animal for Cat {
        fn make_noise(&self)
        {
            println!("Meow!!")
        }
    }

    pub fn demo()
    {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog{}),
            Box::new(Cat{})
        ];
        for item in &animals {
            item.make_noise();
        }
    }
}


pub fn test_all()
{
    example_simple::demo();
}