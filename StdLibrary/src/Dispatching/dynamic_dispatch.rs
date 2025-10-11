

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

mod example2
{
    trait Printable {
        fn print(&self);
    }

    struct StringType {
        name: String
    }

    struct IntegerType {
        value: i32
    }

    impl Printable for StringType
    {
        fn print(&self) {
            println!("StringType(name: {})", self.name);
        }
    }

    impl Printable for IntegerType
    {
        fn print(&self) {
            println!("IntegerType(value: {})", self.value);
        }
    }

    pub fn demo()
    {
        let animals: Vec<Box<dyn Printable>> = vec![
            Box::new(StringType {name: "TestName".to_string() }),
            Box::new(IntegerType {value: 123 })
        ];
        for item in &animals {
            item.print();
        }

        // StringType(name: TestName)
        // IntegerType(value: 123)
    }
}


pub fn test_all()
{
    // example_simple::demo();
    example2::demo();
}