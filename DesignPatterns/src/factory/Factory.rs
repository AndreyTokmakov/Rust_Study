
mod example_one
{
    trait Shape {
        fn draw(&self);
    }

    struct Circle;
    struct Square;

    impl Shape for Circle
    {
        fn draw(&self) {
            println!("Circle");
        }
    }

    impl Shape for Square
    {
        fn draw(&self)  {
            println!("Square");
        }
    }

    fn create_shape(kind: &str) -> Box<dyn Shape>
    {
        match kind {
            "circle" => Box::new(Circle),
            "square" => Box::new(Square),
            _ => panic!("unknown shape"),
        }
    }

    pub fn demo()
    {
        let mut obj: Box<dyn Shape> = create_shape("circle");
        obj.draw();

        obj = create_shape("square");
        obj.draw();
    }
}

pub fn test_all()
{
    example_one::demo();
}