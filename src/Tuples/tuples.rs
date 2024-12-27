
fn create_simple_pair()
{
    let pair = (1, true);
    println!("{:?}", pair);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn reverse_test() {
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



fn structure_binding() {
    let ( first, middle, last ) = ("One", "Two", "Three");
    println!("{} {} {}", first, middle, last );
}

fn structure_binding_1()
{
    let tuple = (1, "hi", 4.5, true);
    let (a, b, c, d) = tuple;

    println!("{} {} {} {}", a, b, c, d);
}


pub fn test_all()
{
    // create_simple_pair();

    // reverse_test();

    // get_tuple_elements();

    // structure_binding();
    structure_binding_1();
}