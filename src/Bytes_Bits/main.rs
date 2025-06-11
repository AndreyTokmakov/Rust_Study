

fn print_as_binary()
{
    let а: u16 = 50115;
    let b: i16 = -15421;

    println! ("а: {:016b} {}", а, а);
    println! ("b: {:016b} {}", b, b);
    
    // а: 1100001111000011 50115
    // b: 1100001111000011 -15421
}


pub fn test_all()
{
    print_as_binary();
}