

mod basics
{
    use std::cell::Cell;

    pub fn demo()
    {
        let intCell: Cell<i32> = Cell::new(5);
        println!("Initial value: {}", intCell.get());

        intCell.set(10);
        println!("Updated value: {}", intCell.get());
    }
}

mod interior_flag_mutability
{
    use std::cell::Cell;

    struct Worker {
        active: Cell<bool>,
    }

    impl Worker
    {
        fn new(initial: bool) -> Self {
            Worker { active: Cell::new(initial) }
        }

        fn start(&self) {
            self.active.set(true);
        }

        fn stop(&self) {
            self.active.set(false);
        }

        fn is_active(&self) -> bool {
            self.active.get()
        }
    }


    pub fn demo()
    {
        let worker: Worker = Worker::new(false);
        println!("Initially active? {}", worker.is_active());

        worker.start();
        println!("Now active? {}", worker.is_active());
    }
}

pub fn test_all()
{
    // basics::demo();
    interior_flag_mutability::demo();
}