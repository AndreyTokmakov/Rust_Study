
mod class_methods;
mod tuple_structs;
mod Long;
mod static_variables_and_methods;
mod inline_methods;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    return User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn create_class_instance() {
    let user: User = build_user(String::from("someone@example.com"),
                                String::from("someusername123"));

    println!("User({}, {})", user.email, user.username)
}

//-------------------------------------------------------------------------------

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // This is a static method
    // Static methods do not need to be called from an instance
    // These methods are usually used as constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method, takes two arguments
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}



fn test_Point() {
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("{:?}", point);

    let top_edge = 1;
    let left_edge = 2;

    println!("\n[left_edge: {}, top_edge: {}]", left_edge, top_edge);

    let Point { x: left_edge, y: top_edge } = point;
    println!("\n[left_edge: {}, top_edge: {}]", left_edge, top_edge);
    println!("{:?}", point);
}

//-------------------------------------------------------------------------------

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn test_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}


//-------------------------------------------------------------------------------

#[derive(Debug)]
struct RectangleDebug {
    width: u32,
    height: u32,
}

fn test_rectangle_print() {
    let rect1 = RectangleDebug {
        width: 10,
        height: 20,
    };

    let scale = 2;
    let rect2 = RectangleDebug {
        width: dbg!(10 * scale),
        height: 20,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect2 is {:?}", rect2);
    println!("rect2 is Rect ({}, {})", rect2.width, rect2.width);
}

//-------------------------------------------------------------------------------


fn printUser(user: &User) {
    println!("User(Email: {}, Name: {}, Active: {}, Count: {})",
             user.email, user.username, user.active, user.sign_in_count)
}

fn create_one_instance_from_another() {
    let user1: User = build_user(String::from("someone@example.com"),
                                  String::from("someusername123"));

    printUser(&user1);

    let user2 = User {
        username: String::from("Jonh Dow"),
        email: String::from("another@example.com"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    printUser(&user2);

    let user3 = User {
        email: String::from("1234@example.com"),
        ..user2
    };

    printUser(&user3);

}

pub fn test_all()
{
    // class_methods::tests();
    inline_methods::test_all();
    // static_variables_and_methods::test_all();
    
    // create_class_instance();
    // create_one_instance_from_another();
    // test_rectangle();
    // test_rectangle_print();

    // test_Point();


    // tuple_structs::tests();

    // Long::tests();
}
