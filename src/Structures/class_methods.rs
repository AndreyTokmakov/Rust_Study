

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_info(&self) {
        println!("Rectangle({}, {})", self.width, self.height);
    }

    fn equals(&self, other: &Rectangle) -> bool {
        self.width == other.width && self.height == other.height
    }

    fn square_default(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn get_rectangle_area() {
    let rect = Rectangle {
        width: 50,
        height: 50,
    };

    let rect3 = Rectangle::square_default(50);

    rect.print_info();
    println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("Are same: {}", rect.equals(&rect3));
}

pub fn tests() {
    get_rectangle_area();
}