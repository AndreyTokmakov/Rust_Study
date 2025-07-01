
#[derive(Debug)]
struct Object {
    data: String,
}

impl Object {
    fn create(d: &str) -> Object {
        println!("Object(data: \"{}\") created", d);
        Object { data: String::from(d) }
    }
}

impl Drop for Object
{
    fn drop(&mut self) {
        println!("Dropping Object {:?}", self);
    }
}

fn makeDefault() -> Object {
    let obj: Object = Object::create("Default");
    return obj
}

mod RAII
{
    use crate::lifetime_borrowing::{makeDefault, Object};

    pub fn create_and_drop()
    {
        let obj: Object  = Object::create("hello");
        println!("Made a ToDrop!");
        
        // Object(data: "hello") created
        // Made a ToDrop!
        // Dropping Object Object { data: "hello" }
    }

    pub fn create_and_drop_1()
    {
        let obj: Object = makeDefault();
        println!("Made a ToDrop!");
        
        // Object(data: "Default") created
        // Made a ToDrop!
        // Dropping Object Object { data: "Default" }
    }
}

mod ownership_borrowing
{
    use crate::lifetime_borrowing::{makeDefault, Object};
    
    fn sink(o: Object)
    {
        println!("sink: {:?}", o);
    }

    pub fn pass_ownership_to_function()
    {
        let obj: Object = makeDefault();
        sink(obj);
        println!("After sink() call");

        // Object(data: "Default") created
        // sink: Object { data: "Default" }
        // Dropping Object Object { data: "Default" }
        // After sink() call
    }
}

mod mutability
{
    use crate::lifetime_borrowing::Object;

    // Mutability of data can be changed when ownership is transferred.
    pub fn change_mutability()
    {
        let obj: Object  = Object::create("hello");

        // *** will not compiler ***
        // obj.data = "hello world".to_string();

        let mut mutable_obj = obj;

        mutable_obj.data = "hello world".to_string();
    }
}

mod partial_moves
{
    use crate::lifetime_borrowing::Object;

    #[derive(Debug)]
    struct SuperObject {
        first: Object,
        second: Object,
    }

    impl SuperObject {
        fn create(first: &str, second: &str) -> SuperObject {
            SuperObject { 
                first: Object::create(first),
                second: Object::create(second)
            }
        }
    }

    /*
    impl Drop for SuperObject
    {
        fn drop(&mut self) {
            println!("Dropping SuperObject {:?}", self);
        }
    }*/
    
    pub fn move_part_of_object()
    {
        let objSuper: SuperObject = SuperObject::create("First", "Second");

        {
            // `first` is moved out of person, but `second` is referenced
            {
                let SuperObject { first, ref second } = objSuper;
            }
            println!("---------------------- End of scope -----------------------");
        }
        
        // Object(data: "First") created
        // Object(data: "Second") created
        // Dropping Object Object { data: "First" }
        // ---------------------- End of scope -----------------------
        // Dropping Object Object { data: "Second" }
    }
}


pub fn test_all()
{
    // RAII::create_and_drop();
    // RAII::create_and_drop_1();
    
    // ownership_borrowing::pass_ownership_to_function();

    // mutability::change_mutability();
    
    partial_moves::move_part_of_object();
}