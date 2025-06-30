
fn test1()
{
    // Assign a reference to the type `i32'. The `&` symbol means that a reference is assigned.
    let reference = &1;

    match reference {
        // If `reference` is a template that is mapped to `&val`,
        // then this will result in a comparison: `&i32`, `&val`
        // We see that if we discard the matched `&`, then the variable `val` should be assigned `i32`.
        &val => println!("We get the value through destructurization: {:?}", val),
    }

    // To avoid the `&` character, you need to dereference the link before matching.
    match *reference {
        val => println!("We get the value through dereference: {:?}", val),
    }

    // INFO: Also a reference
    let ref _is_a_reference = 2;

    match _is_a_reference {
        &val => println!("_is_a_reference : {:?}", _is_a_reference),
    }

    let value = 3;

    // Use the keyword `ref` to create a link.
    match value {
        ref r => println!("Got a link to the value: {:?}", r),
    }
}


pub fn test_all()
{
    test1();
}
