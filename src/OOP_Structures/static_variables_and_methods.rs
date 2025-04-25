
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D 
{
    // This is a static method
    // Static methods do not need to be called from an instance these methods are usually used as constructors
    fn origin() -> Point2D {
        Point2D { x: 0.0, y: 0.0 }
    }

    // Another static method, takes two arguments
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    // Non_Static
    fn setX(&mut self, new_X: f64) -> &mut Self {
        self.x = new_X;
        return self
    }

    // Non_Static
    fn setY(&mut self, new_Y: f64) -> &mut Self {
        self.y = new_Y;
        return self
    }
}


struct MyStruct {
    x: i32,
    y: i32,
}

impl MyStruct {
    const MY_STATIC: i32 = 123;
}


fn static_methods() 
{
    let mut pt: Point2D = Point2D::new(3.0, 4.0);
    println!("{:?}", pt);
    println!("{:?}", pt.setX(3.1).setY(4.1));

    let mut pt1: Point2D = Point2D::origin();
    println!("{:?}", pt1);
    println!("{:?}", pt1.setX(0.1).setY(0.1));
}

fn static_field()
{
    println!("MyStruct::MY_STATIC = {}", MyStruct::MY_STATIC);
}

pub fn test_all()
{
    // static_methods();
    static_field();
}