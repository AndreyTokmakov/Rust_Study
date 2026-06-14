
mod basic
{
    use std::cell::RefCell;
    
    pub fn example()
    {
        let ptr: RefCell<i32> = RefCell::new(5);

        *ptr.borrow_mut() += 1;
        println!("x = {}", ptr.borrow()); // x = 6
    }

    pub fn example_2()
    {
        let data: RefCell<i32> = RefCell::new(5); // Create a RefCell containing a value
        let reference: &RefCell<i32> = &data;           // Create an immutable reference

        // Even though we only have an immutable reference, we can mutate the value inside the RefCell
        println!("Before mutation: {:?}", reference.borrow());

        *reference.borrow_mut() += 10;   // Mutate the value
        println!("After mutation: {:?}", reference.borrow());

        // Before mutation: 5
        // After mutation: 15
    }
}

mod ref_cell_as_class_member
{
    use std::cell::RefCell;

    struct Logger {
        messages: RefCell<Vec<String>>,
    }

    impl Logger {
        fn log(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_string());
        }
        
        fn print(&self) {
            println!("{:?}", self.messages.borrow());
        }
    }

    pub fn example() 
    {
        let logger: Logger = Logger {
            messages: RefCell::new(vec![]),
        };

        logger.log("Start");
        logger.log("Progress...");
        logger.log("End");

        logger.print();
    }
}


mod RefCell_holding_Vec
{
    use std::cell::RefCell;
    
    pub fn example()
    {
        let vecRefCell: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3]);
        vecRefCell.borrow_mut().push(4);
        println!("Vector = {:?}", vecRefCell.borrow());
    }
}


mod Rc_and_RefCell
{
    use std::rc::Rc;
    use std::cell::RefCell;

    pub fn sharded_and_mutable_data()
    {
        // Create a value that is both shared and mutable
        let shared_mutable_data: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![1, 2, 3]));

        // Create a clone of the reference
        let data_clone: Rc<RefCell<Vec<i32>>> = Rc::clone(&shared_mutable_data);

        shared_mutable_data.borrow_mut().push(4); // Modify the original data through the shared reference
        data_clone.borrow_mut().push(5);          // Modify the data through the cloned reference

        // Both references see all modifications
        println!("Shared data: {:?}", shared_mutable_data.borrow());
        println!("Cloned data: {:?}", data_clone.borrow());
    }
}

/**
    RefCell<T> — мутация при immut borrow
    Allows mutation even with an immutable reference.
    ❗ Check for multiple mutations - at runtime, not at compile time.
**/

/**
RefCell<T>: Interior Mutability
RefCell<T>  allows you to mutate data even when there are immutable references to that data,
           through a pattern called "interior mutability.
Unlike Box<T> and Rc<T>, RefCell<T> enforces borrowing rules at runtime instead of compile time.

When to use RefCell<T>:
When you need to mutate data that's behind an immutable reference
When you're certain your code follows borrowing rules but the compiler can't verify it
In combination with Rc<T> to create data that can be shared and mutated
**/
pub fn test_all()
{
    // basic::example();
    // basic::example_2();

    Rc_and_RefCell::sharded_and_mutable_data();

    // ref_cell_as_class_member::example();
    // RefCell_holding_Vec::example();
}