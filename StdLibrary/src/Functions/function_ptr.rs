

mod example_1
{
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn demo()
    {
        let f: fn(i32, i32) -> i32 = add;
        println!("2 + 3 = {}", f(2, 3));
    }
}

mod example_2
{
    type FnPtr = fn() -> String;

    fn add_field() -> String {
        "add field".to_string()
    }

    fn remove_field() -> String {
        "remove field".to_string()
    }

    pub fn demo()
    {
        let ptrFun: FnPtr = add_field;
        println!("ptrFun: {}", ptrFun());

        let ptrFun: FnPtr = remove_field;
        println!("ptrFun: {}", ptrFun());
    }
}



pub fn test_all()
{
    // example_1::demo();
    example_2::demo();
}