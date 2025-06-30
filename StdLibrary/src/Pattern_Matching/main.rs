
mod while_let;

mod matching_literals
{
    pub fn demo()
    {
        let x: i32 = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
}

mod matching_named_variables
{
    fn matcher(x: Option<i32>, y: i32)
    {
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }
        println!("at the end: x = {x:?}, y = {y}");
    }
    
    pub fn demo()
    {
        matcher(Some(5), 10);
        matcher(None, 123);
        matcher(Some(50), 10);
    }
}


mod multiple_patterns
{
    fn check(value: i32)
    {
        match value {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    
    pub fn demo()
    {
        check(3);
        check(1);
        check(10);
    }
}


mod range_of_values
{
    use serde_json::map::VacantEntry;

    fn check(value: i32)
    {
        match value {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }
    
    fn check_char(ch: char)
    {
        match ch {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    pub fn demo()
    {
        check(2);
        check(3);
        check(5);
        check(6);

        check_char('b');
        check_char('j');
        check_char('t');
    }
}

mod match_custom_class
{
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn check(pt: Point)
    {
        match pt {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    pub fn demo()
    {
        check(Point{x: 1, y: 0});
        check(Point{x: 0, y: 1});
        check(Point{x: 1, y: 1});
    }
}

pub fn test_all()
{
    // matching_literals::demo();
    // matching_named_variables::demo();
    // multiple_patterns::demo();
    // range_of_values::demo();
    // match_custom_class::demo();
    
    while_let::test_all();
}