

pub fn tests() {
    test_enum_method();
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        print!("Enum method called\n");
    }
}

fn test_enum_method() {
    let m1 = Message::Write(String::from("hello"));
    m1.call();

    let m2 = Message::Quit;
    m2.call();
}
