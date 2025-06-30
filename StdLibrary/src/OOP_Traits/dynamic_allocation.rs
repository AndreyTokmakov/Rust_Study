
struct Square {}
struct Rectangle {}

trait Shape {
    fn info(&self) -> &'static str;
}

impl Shape for Square {
    fn info(&self) -> &'static str {
        "Square"
    }
}

impl Shape for Rectangle {
    fn info(&self) -> &'static str {
        "Rectangle"
    }
}

fn print_info(shape_impl: Box<dyn Shape>) 
{
    println!("{}", shape_impl.info());
}

pub fn test_all()
{
    let square: Box<Square> = Box::new(Square {});
    let rectangle: Box<Rectangle> = Box::new(Rectangle {});

    print_info(square);
    print_info(rectangle);
}