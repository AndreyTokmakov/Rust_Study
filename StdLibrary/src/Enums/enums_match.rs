
#[derive(Debug)]
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter,
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
    for val in [CoinWithState::Penny, 
                             CoinWithState::Nickel,
                             CoinWithState::Dime,
                             CoinWithState::Quarter(UsState::Alaska), 
                             CoinWithState::Quarter(UsState::Alabama)] 
    {
        coin_with_state_to_cents(val);
    }
}


mod example_4
{
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },    // Named fields (struct-like)
        Write(String),              // Single value (tuple-like)
        ChangeColor(i32, i32, i32), // Multiple values (tuple-like)
    }

    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("Quitting the application"),
            Message::Move { x, y } => println!("Moving to position: ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to RGB: ({}, {}, {})", r, g, b),
        }
    }

    pub fn process_collection()
    {
        let messages = [
            Message::Quit,
            Message::Move { x: 10, y: 5 },
            Message::Write(String::from("Hello, Rust!")),
            Message::ChangeColor(255, 0, 255),
        ];

        for msg in messages {
            process_message(msg);
        }

        // Quitting the application
        // Moving to position: (10, 5)
        // Text message: Hello, Rust!
        // Changing color to RGB: (255, 0, 255)
    }
}


mod example_5
{
    enum Weather
    {
        Sunny,
        Cloudy,
        Rainy(f32), // Amount of rainfall in inches
        Snowy(f32), // Amount of snowfall in inches
    }

    fn get_activity(weather: Weather) -> String
    {
        match weather {
            Weather::Sunny => "Sunny : Go for a hike!".to_string(),
            Weather::Cloudy => String::from("Cloudy: Perhaps read a book outside."),
            Weather::Rainy(amount) if amount < 1.0 => String::from("Rainy : A light walk with an umbrella."),
            Weather::Rainy(_) => String::from("Rainy : Stay inside and watch a movie."),
            Weather::Snowy(amount) if amount < 2.0 => String::from("Snowy : Build a small snowman."),
            Weather::Snowy(_) => String::from("Snowy : Best to stay warm indoors."),
        }
    }

    pub fn match_with_if()
    {
        let forecasts: [Weather; 6] = [
            Weather::Sunny,
            Weather::Cloudy,
            Weather::Rainy(0.5),
            Weather::Rainy(2.3),
            Weather::Snowy(1.0),
            Weather::Snowy(5.0),
        ];

        for forecast in forecasts {
            let activity = get_activity(forecast);
            println!("{}", activity);
        }

        // Sunny : Go for a hike!
        // Cloudy: Perhaps read a book outside.
        // Rainy : A light walk with an umbrella.
        // Rainy : Stay inside and watch a movie.
        // Snowy : Build a small snowman.
        // Snowy : Best to stay warm indoors.
    }
}

pub fn tests()
{
    // switch_enums();
    // switch_enums_print();
    // switch_enums_with_state();

    // example_4::process_collection();
    example_5::match_with_if();
}
