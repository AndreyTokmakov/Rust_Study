
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

mod associated_functions // Associated function (no self parameter)
{
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight
    {
        fn new_red() -> TrafficLight {
            TrafficLight::Red
        }

        fn new_yellow() -> TrafficLight {
            TrafficLight::Yellow
        }

        fn new_green() -> TrafficLight {
            TrafficLight::Green
        }

        // Method (uses self)
        fn get_wait_time(&self) -> u32 {
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 45,
            }
        }
    }

    pub fn demo()
    {
        // Using associated functions to create instances
        let red_light: TrafficLight = TrafficLight::new_red();
        let yellow_light: TrafficLight  = TrafficLight::new_yellow();

        println!("Wait time for red light: {} seconds", red_light.get_wait_time());
        println!("Wait time for yellow light: {} seconds", yellow_light.get_wait_time());
    }
}

mod methods_with_pattern_matching
{
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T, E> Result<T, E>
    {
        fn is_ok(&self) -> bool {
            match self {
                Result::Ok(_) => true,
                Result::Err(_) => false,
            }
        }

        fn is_err(&self) -> bool {
            !self.is_ok()
        }

        fn unwrap(self) -> T where E: std::fmt::Debug {
            match self {
                Result::Ok(value) => value,
                Result::Err(err) => panic!("Called unwrap on an Err value: {:?}", err),
            }
        }
    }

    // One of the most powerful applications of enum methods is pattern matching within the method body.
    // This allows you to provide different implementations depending on the variant:
    pub fn demo()
    {
        let ok_result: Result<i32, &str> = Result::Ok(42);
        let err_result: Result<i32, &str> = Result::Err("Something went wrong");

        println!("ok_result is_ok: {}",   ok_result.is_ok());   // Output: ok_result is_ok: true
        println!("err_result is_err: {}", err_result.is_err()); // Output: err_result is_err: true

        // Safely unwrap the Ok value
        let value: i32 = ok_result.unwrap();
        println!("Unwrapped value: {}", value); // Output: Unwrapped value: 42

        // This would panic
        // let value = err_result.unwrap();
    }
}


pub fn tests()
{
    // simple_example::demo();
    // two_methods::demo();
    // associated_functions::demo();
    methods_with_pattern_matching::demo();
}