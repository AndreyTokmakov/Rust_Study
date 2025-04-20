
mod enums_methods;
mod enums_match;
mod cpp_style_enums;
mod underlying_type;

#[derive(Debug)]
enum ProtocolVersion
{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr
{
    version: ProtocolVersion,
    address: String,
}

fn basic_enum_demo()
{
    let home = IpAddr {
        version: ProtocolVersion::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        version: ProtocolVersion::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}



#[derive(Debug)]
enum ProtocolVersionEx {
    V4(String),
    V6(String),
}

fn sting_enum_demo()
{
    let home = ProtocolVersionEx::V4(String::from("127.0.0.1"));
    let loopback = ProtocolVersionEx::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}

#[derive(Debug)]
enum IpVersion
{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn diff_types_enum_demo()
{
    let home: IpVersion = IpVersion::V4(127, 0, 0, 1);
    let loopback: IpVersion = IpVersion::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}



pub fn test_all()
{
    // basic_enum_demo();
    // sting_enum_demo();
    // diff_types_enum_demo();

    // underlying_type::tests();

    enums_methods::tests();
    // enums_match::tests();

    // cpp_style_enums::tests();
}
