

#[repr(u8)]
enum FieldlessWithDiscrimants {
    First = 10,
    Tuple(),
    Second = 20,
    Struct{},
    Unit,
}

fn validate_size()
{
    assert_eq!(10, FieldlessWithDiscrimants::First as u8);
    assert_eq!(11, FieldlessWithDiscrimants::Tuple() as u8);
    assert_eq!(20, FieldlessWithDiscrimants::Second as u8);
    assert_eq!(21, FieldlessWithDiscrimants::Struct{} as u8);
    assert_eq!(22, FieldlessWithDiscrimants::Unit as u8);
}

#[repr(u8)]
enum Enum {
    Unit,
    Tuple(bool),
    Struct{a: bool},
}


impl Enum
{
    fn discriminant(&self) -> u8
    {
        unsafe { *(self as *const Self as *const u8) }
    }
}

// https://doc.rust-lang.org/reference/items/enumerations.html
pub fn tests()
{
    // validate_size();

    println!("{:#?}", Enum::Unit.discriminant());
    println!("{:#?}", Enum::Tuple(true).discriminant());
    println!("{:#?}", Enum::Struct{a: false}.discriminant());
}