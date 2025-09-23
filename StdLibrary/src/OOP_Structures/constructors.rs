

mod simple
{
    struct Object
    {
        value: u64,
    }

    impl Object
    {
        pub fn new(value: u64) -> Self {
            Self { value }
        }

        pub fn value(&self) -> u64 {
            self.value
        }
    }

    pub fn demo()
    {
        let obj: Object = Object::new(123);
        println!("{}", obj.value());
    }
}

mod default_constructor
{
    struct Object {
        value: u64,
    }

    impl Object {
        pub fn value(&self) -> u64 {
            self.value
        }
    }

    impl Default for Object {
        fn default() -> Self {
            Self { value: 0 }
        }
    }

    pub fn demo()
    {
        let obj: Object = Object::default();
        println!("{}", obj.value());
    }
}

mod default_constructors_derived
{
    #[derive(Default)]
    struct Object {
        value: u64,
    }

    impl Object {
        pub fn value(&self) -> u64 {
            self.value
        }
    }

    pub fn demo()
    {
        let obj: Object = Object::default();
        println!("{}", obj.value());
    }
}

// https://rust-unofficial.github.io/patterns/idioms/ctor.html
pub fn test_all()
{
    // simple::demo();
    //default_constructor::demo();
    default_constructors_derived::demo();
}
