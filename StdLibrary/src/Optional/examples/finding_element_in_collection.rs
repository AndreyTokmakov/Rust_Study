
mod finding_element_in_collection
{
    pub fn demo()
    {
        let numbers: Vec<i32> = vec![10, 20, 30, 40, 50];
        let found: Option<&i32> = numbers.iter().find(|&&x| x == 30);

        match found {
            Some(&value) => println!("Found the value: {}", value),
            None => println!("Value not found"),
        }
        // Output: Found the value: 30


        // Using methods on Option for a more concise approach
        let not_found: Option<&i32> = numbers.iter().find(|&&x| x == 100);
        let message: &str = not_found.map_or("Value not found",
            |&x| "Error"
        );
        println!("{}", message);
    }
}

pub fn test_all()
{
    finding_element_in_collection::demo();
}