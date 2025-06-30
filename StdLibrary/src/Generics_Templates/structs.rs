

struct Point<T> {
    x: T,
    y: T,
}

struct PointEx<T, U> {
    x: T,
    y: U,
}

struct Object<T>
{
    value: T
}

impl<T> Object<T>
{
    fn getValue(&self) -> &T {
        &self.value
    }
}

fn class_method()
{
    {
        let v: Object<i32> = Object {value: 12345};
        let val = v.getValue();
        println!("{}", val);
    }

    {
        let v: Object<String> = Object {value: String::from("Some_String_Value")};
        let val = v.getValue();
        println!("{}", val);
    }
}

fn simple_Point_struct_test()
{
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixedTypes = PointEx { x: 5, y: 10.1 };
}


pub fn test_all()
{
    // simple_Point_struct_test();
    class_method();
}