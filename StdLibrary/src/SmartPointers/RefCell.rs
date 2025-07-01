
mod basic
{
    use std::cell::RefCell;
    
    pub fn example()
    {
        let ptr: RefCell<i32> = RefCell::new(5);

        *ptr.borrow_mut() += 1;
        println!("x = {}", ptr.borrow()); // x = 6
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


/**
    RefCell<T> — мутация при immut borrow
    Allows mutation even with an immutable reference.
    ❗ Check for multiple mutations - at runtime, not at compile time.
**/
pub fn test_all()
{
    // basic::example();
    // ref_cell_as_class_member::example();
    RefCell_holding_Vec::example();

}