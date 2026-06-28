
pub mod observer_wrapper
{
    use std::cell::{Ref, RefCell};

    struct Observable<T>
    {
        value: RefCell<T>,
        observers: Vec<fn(&T)>,
    }

    impl<T: Clone> Observable<T>
    {
        fn new(initial: T) -> Self {
            Observable {
                value: RefCell::new(initial),
                observers: Vec::new(),
            }
        }

        fn add_observer(&mut self, observer: fn(&T)) {
            self.observers.push(observer);
        }

        fn set(&self, new_value: T)
        {
            *self.value.borrow_mut() = new_value;
            let value: Ref<T> = self.value.borrow();
            for observer in &self.observers {
                observer(&value);
            }
        }

        fn get(&self) -> T {
            self.value.borrow().clone()
        }
    }

    pub fn test_all()
    {
        let mut counter: Observable<i32> = Observable::new(0);

        counter.add_observer(|value| println!("Observer 1: Value changed to {}", value));
        counter.add_observer(|value| println!("Observer 2: Value is now {}", value));

        println!("Initial value: {}", counter.get());
        counter.set(42);
        counter.set(100);

        // Initial value: 0
        // Observer 1: Value changed to 42
        // Observer 2: Value is now 42
        // Observer 1: Value changed to 100
        // Observer 2: Value is now 100
    }
}
