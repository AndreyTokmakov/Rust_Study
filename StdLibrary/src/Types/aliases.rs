
pub fn type_aliases_tests() {
    test();
}


// `NanoSecond` is a new name for `u64'.
type NanoSecond = u64;
type Inch = u64;

// Use this attribute to avoid displaying a warning about names not in the camelCase style
#[allow(non_camel_case_types)]
type u64_t = u64;


fn test()
{
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that aliases *do not provide* any
    // additional type security, since *are not*new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}