

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn print_info(&self)
    {
        println!("Rectangle({}, {})", self.width, self.height);
    }

    fn equals(&self, other: &Rectangle) -> bool
    {
        self.width == other.width && self.height == other.height
    }

    /// Static fabric method to create
    fn create(w: u32, h: u32) -> Rectangle
    {
        Rectangle {
            width: w,
            height: h,
        }
    }

    /// Static fabric method to create a Square
    fn createSquare(size: u32) -> Rectangle
    {
        Rectangle::create(size, size)
    }
}

fn get_rectangle_area()
{
    let square: Rectangle = Rectangle::createSquare(50);
    let rect1 = Rectangle::create(40, 60);
    let rect2 = Rectangle::create(50, 50);

    square.print_info();
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Are same: {}", square.equals(&rect2));
}

/**
The &self Parameter

    The first parameter of a method is always self in some form,
    which represents the instance of the struct the method is being called on:

    &self     - borrows the instance immutably (most common)
    &mut self - borrows the instance mutably
    self      - takes ownership of the instance (rare)

Using &self means we're borrowing the struct instance immutably - we can read its data but not modify it.
**/

pub fn tests()
{
    get_rectangle_area();
}