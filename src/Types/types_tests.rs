
mod aliases;
mod From_Into;
mod Try_From;
mod Try_Into;
mod From_To_String;
mod newtype_idiom;
mod global_variables;


fn variable_scope()
{
    // This variable lives in the main function
    let long_lived_binding = 1;

    {
        // This variable exists only in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // Error! `short_lived_binding` is not in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}

fn shadowing() {
    let shadowed_binding = 1;

    {
        println!("Before shadowing: {}", shadowed_binding);

        // shadowing
        let shadowed_binding = "abc";

        println!("Inner 'shadowing': {}", shadowed_binding);
    }
    println!("Outer 'shadowing': {}", shadowed_binding);

    // // This binding *obscures* the previous one
    let shadowed_binding = 2;
    println!("затенённая во внешнем блоке: {}", shadowed_binding);
}

fn type_casting() {
    let decimal = 65.4321_f32;

    // FIXME: Error!!
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("integer {} character = {}", integer, character);
    println!("Cast: {} -> {} -> {}", decimal, integer, character);


    // println!("1000 as u8: {}", 1000 as u8);
    // println!(" 232 as i8: {}", 232 as i8);
}

fn literals()
{
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Literals without suffixes. Their type will depend on how they are used.
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of the variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}


pub fn test_all()
{
    global_variables::test_all();

    // variable_scope();

    // shadowing();

    // type_casting();

    // literals();

    // aliases::type_aliases_tests();

    // From_Into::test_all();

    // Try_From::test_all();
    // Try_Into::test_all();
    // newtype_idiom::test_all();

    // From_To_String::test_all();
}