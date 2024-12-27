
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest_element()
{
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is '{}'", result);

    let char_list = vec!['y', 'm', 'a', 'q' , 'z'];

    let result = largest(&char_list);
    println!("The largest char is '{}'", result);
}

//--------------------------------------------------------------------------------

struct Point<T> {
    x: T,
    y: T,
}

struct PointEx<T, U> {
    x: T,
    y: U,
}

fn simple_Point_struct_test() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let mixedTypes = PointEx { x: 5, y: 10.1 };
}

//--------------------------------------------------------------------------------

struct Object<T> {
    value: T
}

impl<T> Object<T> {
    fn getValue(&self) -> &T {
        &self.value
    }
}

fn class_method() {

    {
        let v: Object<i32> = Object {value: 5};
        let val = v.getValue();

        println!("{}", val);
    }

    {
        let v: Object<String> = Object {value: String::from("Some_String_Value")};
        let val = v.getValue();

        println!("{}", val);
    }
}


//--------------------------------------------------------------------------------

struct ObjectEx<T> {
    value: T
}

impl ObjectEx<i32> {
    fn getValue(&self) -> &i32 {
        &self.value
    }
}

impl ObjectEx<String> {
    fn getValue(&self) -> String  {
        String::from("String_Value!!!!")
    }
}

fn template_specialization() {
    {
        let v: ObjectEx<i32> = ObjectEx {value: 5};
        let val = v.getValue();

        println!("{}", val);
    }


    {
        let v: ObjectEx<String> = ObjectEx {value: String::from("")};
        let val = v.getValue();

        println!("{}", val);
    }
}

pub fn test_all()
{
    // find_largest_element();
    // simple_Point_struct_test();

    // class_method();

    template_specialization();
}