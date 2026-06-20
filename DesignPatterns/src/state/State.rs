

mod simple_example
{
    struct Start;
    struct InProgress;
    struct Done;

    trait State {
        fn next(self: Box<Self>) -> Box<dyn State>;
        fn name(&self) -> &'static str;
    }

    impl State for Start {
        fn next(self: Box<Self>) -> Box<dyn State> {
            Box::new(InProgress)
        }
        fn name(&self) -> &'static str {
            "Start"
        }
    }

    impl State for InProgress {
        fn next(self: Box<Self>) -> Box<dyn State> {
            Box::new(Done)
        }
        fn name(&self) -> &'static str {
            "InProgress"
        }
    }

    impl State for Done {
        fn next(self: Box<Self>) -> Box<dyn State> {
            Box::new(Done)
        }
        fn name(&self) -> &'static str {
            "Done"
        }
    }

    pub fn demo() 
    {
        let mut state: Box<dyn State> = Box::new(Start);
        for _ in 0..3 {
            println!("Текущее состояние: {}", state.name());
            state = state.next();
        }
    }
}


pub fn test_all()
{
    simple_example::demo();
}