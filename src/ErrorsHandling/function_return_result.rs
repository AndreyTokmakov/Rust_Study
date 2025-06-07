
fn dev_as_string(a: i32, b: i32) -> Result<String, String> {
    if (0 == b) {
        Err(String::from("Error: b == 0"))
    }
    else {
        Ok((a/b).to_string())
    }
}

pub fn return_result_or_err()
{
    {
        let res = dev_as_string(10, 2);
        println!("{}", res.unwrap());
    }
    {
        let res = dev_as_string(10, 0);
        println!("{}", res.unwrap());
    }
}


pub fn test_all()
{
    return_result_or_err();
}