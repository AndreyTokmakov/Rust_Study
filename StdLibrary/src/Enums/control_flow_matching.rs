

fn check_variable()
{
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

fn check_variable_if_init()
{
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

pub fn test_all()
{
    // check_variable();
    check_variable_if_init();   
}