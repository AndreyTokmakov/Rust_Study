
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn simple_test() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


pub fn tests_all() {
    simple_test();
}