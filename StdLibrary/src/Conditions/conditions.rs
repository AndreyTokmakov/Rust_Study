

pub fn test_all()
{
    test0();
    // test1();
}

fn test0()
{
    let condition = true;
    if true == condition {
        println!("True");
    }
    else {
        println!("False");
    }
}

fn test1()
{
    let condition = true;
    let result = if condition { 1 } else { 0 };
    println!("result = {}", result);
}