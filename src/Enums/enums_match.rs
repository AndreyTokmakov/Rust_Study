
pub fn tests() {
    // switch_enums();
    // switch_enums_ex();
    handle_Other_Values__DEFAULT();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_Ex(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn switch_enums() {
    let coin: Coin = Coin::Nickel;
    let val: u8 = value_in_cents(coin);
    println!("Coin: {}", val);
}

fn switch_enums_ex() {
    let coin: Coin = Coin::Penny;
    let val: u8 = value_in_cents_Ex(coin);
    println!("Coin: {}", val);
}

//-------------------------------------------------

fn get_result(number: i32) -> i32 {
    match number {
        1 => action1(),
        2 => action2(),
        other => actionOther(number),
    }
}

fn action1() -> i32 {
    1
}

fn action2() -> i32 {
    2
}

fn actionOther(number: i32) -> i32 {
    println!("Handling unexpected value {}", number);
    number
}

fn handle_Other_Values__DEFAULT() {

    println!("Result: {}", get_result(1));
    println!("Result: {}", get_result(2));
    println!("Result: {}", get_result(3));
}