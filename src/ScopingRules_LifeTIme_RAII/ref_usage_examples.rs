
#[derive(Debug, Clone, Copy)]
struct Point
{ 
    x: i32, 
    y: i32
}

fn simple_borrow()
{
    let c: char = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
}

fn destructuring_struct()
{
    let point = Point { x: 10, y: 20 };

    // `ref` is also valid when destructuring a struct.
    let copy_of_x: i32 = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    println!("copy_of_x ({})", copy_of_x);
    println!("point is ({:?})", point);
}


fn take_mutable_references()
{
    let point: Point = Point { x: 10, y: 20 };
    let mut mutable_point: Point = point;   // A mutable copy of `point`

    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;
        // Mutate the `y` field of `mutable_point` via a mutable reference.
        *mut_ref_to_y = 12345;
    }

    println!("point is ({:?})", point);
    println!("mutable_point is ({:?})", mutable_point);
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
    // simple_borrow();
    // destructuring_struct();
    // take_mutable_references();
    mutable_tuple_with_pointer();
}