
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn test1()
{
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn test2()
{
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("----------------------------------------------------");
    drop(c);
    println!("----------------------------------------------------");
}

pub fn test_all()
{
    // test1();
    test2();
}