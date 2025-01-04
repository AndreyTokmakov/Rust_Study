
#[derive(Debug)]
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn to_cents(coin: &Coin) -> u8
{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn coin_to_cents(coin: Coin)
{
    println!("{:?} is equal to {} cents", coin, to_cents(&coin));
}

fn switch_enums()
{
    for val in [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        coin_to_cents(val);
    }
}

fn to_cents_print(coin: &Coin) -> u8
{
    match coin {
        Coin::Penny => {
            println!("* * * * * Lucky penny * * * * *");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn coin_to_cents_print(coin: Coin)
{
    println!("{:?} is equal to {} cents", coin, to_cents_print(&coin));
}

fn switch_enums_print()
{
    for val in [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        coin_to_cents_print(val);
    }
}



#[derive(Debug)]
enum UsState
{
    Alabama,
    Alaska
}

#[derive(Debug)]
enum CoinWithState
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn to_cents_with_state(coin: &CoinWithState) -> u8
{
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("\tState quarter from {state:?}!");
            match state {
                UsState::Alabama => 26,
                UsState::Alaska => 27,
            }
        }
    }
}

fn coin_with_state_to_cents(coin: CoinWithState)
{
    println!("{:?} is equal to {} cents", coin, to_cents_with_state(&coin));
}

fn switch_enums_with_state()
{
    for val in [CoinWithState::Penny, CoinWithState::Nickel, CoinWithState::Dime,
                             CoinWithState::Quarter(UsState::Alaska), CoinWithState::Quarter(UsState::Alabama)] {
        coin_with_state_to_cents(val);
    }
}


pub fn tests()
{
    // switch_enums();
    // switch_enums_print();
    switch_enums_with_state();

    // handle_Other_Values__DEFAULT();
}





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