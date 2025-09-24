

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

mod default_complex_example
{
    use std::{path::PathBuf, time::Duration};

    #[derive(Default, Debug, PartialEq)]
    struct MyConfiguration
    {
        // Option defaults to None
        output: Option<PathBuf>,

        // Vecs default to empty vector
        search_path: Vec<PathBuf>,

        // Duration defaults to zero time
        timeout: Duration,

        // bool defaults to false
        check: bool,
    }

    impl MyConfiguration {
        // add setters here
    }

    pub fn demo()
    {
        // construct a new instance with default values
        let mut conf = MyConfiguration::default();

        // do something with conf here
        conf.check = true;
        println!("conf = {conf:#?}");

        // partial initialization with default values, creates the same instance
        let conf1 = MyConfiguration {
            check: true,
            ..Default::default()
        };
        assert_eq!(conf, conf1);

        // conf = MyConfiguration {
        //     output: None,
        //     search_path: [],
        //     timeout: 0ns,
        //     check: true,
        // }
    }
}

// https://rust-unofficial.github.io/patterns/idioms/ctor.html
pub fn test_all()
{
    // simple::demo();
    //default_constructor::demo();
    // default_constructors_derived::demo();
    default_complex_example::demo();
}
