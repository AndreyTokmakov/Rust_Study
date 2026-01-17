
mod simple_example
{
    #[derive(Debug)]
    enum Message
    {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message
    {
        fn call(&self)
        {
            print!("Enum method called for {:?}\n", self);
        }
    }

    pub fn demo()
    {
        let messages: [Message; 4] = [
            Message::Write(String::from("hello")),
            Message::Quit,
            Message::Move {x: 1, y: 2},
            Message::ChangeColor(1,2,3),
        ];

        for msg in messages {
            msg.call();
        }

        // Enum method called for Write("hello")
        // Enum method called for Quit
        // Enum method called for Move { x: 1, y: 2 }
        // Enum method called for ChangeColor(1, 2, 3)
    }
}


mod two_methods
{
    enum Shape {
        Circle(f64),             // radius
        Rectangle(f64, f64),     // width, height
        Triangle(f64, f64, f64), // three sides
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Rectangle(width, height) => width * height,
                Shape::Triangle(a, b, c) => {
                    // Heron's formula
                    let s = (a + b + c) / 2.0;
                    (s * (s - a) * (s - b) * (s - c)).sqrt()
                }
            }
        }

        fn name(&self) -> String {
            match self {
                Shape::Circle(_) => String::from("circle"),
                Shape::Rectangle(_, _) => String::from("rectangle"),
                Shape::Triangle(_, _, _) => String::from("triangle"),
            }
        }
    }

    pub fn demo()
    {
        let shapes: [Shape; 3] = [
            Shape::Circle(5.0),
            Shape::Rectangle(4.0, 6.0),
            Shape::Triangle(3.0, 4.0, 5.0),
        ];

        for shape in shapes {
            println!("area of '{}' is {:.2}", shape.name(), shape.area());
        }

        // area of 'circle' is 78.54
        // area of 'rectangle' is 24.00
        // area of 'triangle' is 6.00
    }
}


pub fn tests()
{
    // simple_example::demo();
    two_methods::demo();
}