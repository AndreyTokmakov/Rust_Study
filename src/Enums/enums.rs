
mod enums_methods;
mod enums_match;
mod cpp_style_enums;

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn test1()
{
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

//----------------------------------------------------------- Enums with String

#[derive(Debug)]
enum IpAddrEx {
    V4(String),
    V6(String),
}

fn test2()
{
    let home = IpAddrEx::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEx::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}

//----------------------------------------------------------- Enums diff types

enum IpAddrEx2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn test3()
{
    let home = IpAddrEx2::V4(127, 0, 0, 1);
    let loopback = IpAddrEx2::V6(String::from("::1"));
}



pub fn test_all()
{
    // println!("Enums tests");

    // test1();
    // test2();
    // test3();

    // enums_methods::tests();

    // enums_match::tests();

    cpp_style_enums::tests();
}
