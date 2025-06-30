
struct Point
{
    x: i32,
    y: i32,
    z: i32
}

pub fn test_all()
{
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Error! Can't borrow `point` as mutable because it's currently borrowed as immutable.
    //     let mutable_borrow = &mut point;
    // TODO : Try uncommenting this line

    // Data can be accessed via the references and the original owner
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);
    
    // The immutable references are no longer used for the rest of the code so it is possible to reborrow with a mutable reference.
    // TODO : OK because the 'borrowed_point' and 'another_borrow' are no longer exist
    let mutable_borrow = &mut point;
    
    // Change data via mutable reference
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
}