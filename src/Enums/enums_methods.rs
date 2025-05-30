

pub fn tests()
{
    test_enum_method();
}

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

fn test_enum_method()
{
    let msgWrite: Message = Message::Write(String::from("hello"));
    msgWrite.call();

    let msgMessage: Message = Message::Quit;
    msgMessage.call();

    let msgMove: Message = Message::Move {x: 1, y: 2};
    msgMove.call();

    let msgChangeColor: Message = Message::ChangeColor(1,2,3);
    msgChangeColor.call();
}
