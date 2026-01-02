
fn create_simple_pair()
{
    let pair: (i32, bool) = (1, true);
    println!("{:?}", pair);
}

fn reverse(pair: (i32, bool)) -> (bool, i32)
{
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn reverse_test()
{
    let (a, b): (i32, bool) = (10, true);
    let (boolean, integer) = reverse((a, b));
    println!("({}, {}) --> ({}, {})", a, b, boolean, integer);
}


fn get_tuple_elements() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    println!("{} {} {} {} {}",
             long_tuple.0, long_tuple.1, long_tuple.2, long_tuple.3, long_tuple.4);
}



fn structure_binding()
{
    let ( first, middle, last ) = ("One", "Two", "Three");
    println!("{} {} {}", first, middle, last );
}

fn structure_binding_1()
{
    let tuple = (1, "hi", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{} {} {} {}", a, b, c, d);
}

fn structure_binding_2()
{
    let tuple: (i32, &str, f32, bool) = (1, "hi", 4.5, true);

    let first: i32 = tuple.0;
    let second: &str = tuple.1;
    let third: f32 = tuple.2;
    let fourth: bool = tuple.3;

    println!("{} {} {} {}", first, second, third, fourth);
}

fn mutable_tuple_with_pointer()
{
    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}



pub fn test_all()
{
    // create_simple_pair();
    // reverse_test();
    // get_tuple_elements();
    // mutable_tuple_with_pointer();

    // structure_binding();
    // structure_binding_1();
    structure_binding_2();
}