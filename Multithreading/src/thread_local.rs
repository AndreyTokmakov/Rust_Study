
mod example_one
{
    use std::cell::RefCell;
    use std::thread::JoinHandle;

    thread_local!
    {
        // Thread-local counter, unique per thread
        static COUNTER: RefCell<u32> = RefCell::new(0);
    }

    fn increment()
    {
        COUNTER.with(|c| {
            *c.borrow_mut() += 1;
        });
    }

    fn get() -> u32
    {
        COUNTER.with(|c| {
            *c.borrow()
        })
    }

    pub fn run()
    {
        let mut handles= Vec::new();
        for i in 0..3 {
            let handle = std::thread::spawn(move || {
                for _ in 0..5 {
                    increment();
                }
                let value: u32 = get();
                println!("Thread {} counter = {}", i, value);
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}

mod example_two
{
    use std::cell::{RefCell, RefMut};

    thread_local!
    {
        // Reusable buffer per thread
        static BUFFER: RefCell<String> = RefCell::new(String::with_capacity(1024));
    }

    fn format_message(id: usize, msg: &str) -> String
    {
        BUFFER.with(|buf| {
            let mut buf: RefMut<String> = buf.borrow_mut();

            buf.clear();
            buf.push_str("Thread ");
            buf.push_str(&id.to_string());
            buf.push_str(": ");
            buf.push_str(msg);

            buf.clone()
        })
    }

    pub fn run()
    {
        let mut handles = Vec::new();
        for i in 0..2
        {
            let handle = std::thread::spawn(move || {
                let s1: String = format_message(i, "hello");
                let s2: String = format_message(i, "world");

                println!("{}", s1);
                println!("{}", s2);
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}

mod example_fast_counter
{
    use std::cell::Cell;

    thread_local!  {
        // Cell is faster for Copy types
        static FAST_COUNTER: Cell<u64> = Cell::new(0);
    }

    fn tick()
    {
        FAST_COUNTER.with(| counter| {
            counter.set(counter.get() + 1);
        });
    }

    pub fn run()
    {
        let handle = std::thread::spawn(move || {
            for _ in 0..10 {
                tick();
            }

            FAST_COUNTER.with(|c| {
                println!("FAST_COUNTER = {}", c.get());
            });
        });
        handle.join().unwrap();
    }
}

pub fn test_all()
{
    // example_one::run();
    // example_two::run();
    example_fast_counter::run();
}