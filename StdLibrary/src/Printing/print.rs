
mod pretty_table;
mod logger_printer;

mod formating
{
    #[derive(Debug)]
    struct User
    {
        id: u32,
        name: String,
        score: f64,
    }

    pub fn edges_alignment()
    {
        let text = "Rust";
        println!("Left:  |{:<10}|", text);
        println!("Right: |{:>10}|", text);
        println!("Center:|{:^10}|", text);

        // Left:  |Rust      |
        // Right: |      Rust|
        // Center:|   Rust   |
    }

    pub fn width_alignment()
    {
        let name = "Alice";
        let age = 30;
        let city = "New York";

        // < - выравнивание по левому краю , > - выравнивание по правому краю , ^ - выравнивание по центру
        println!("{:<10} | {:>5} | {:^15}", "Name", "Age", "City");
        println!("{:-<10}-+-{:-<5}-+-{:-<15}", "", "", "");

        println!("{:<10} | {:>5} | {:^15}", name, age, city);
        println!("{:<10} | {:>5} | {:^15}", "Bob", 22, "London");
        println!("{:<10} | {:>5} | {:^15}", "Charlie", 41, "Paris");

        // Name       |   Age |      City
        // -----------+-------+----------------
        // Alice      |    30 |    New York
        // Bob        |    22 |     London
        // Charlie    |    41 |      Paris
    }

    pub fn fixed_width()
    {
        let id = 42;
        let value = std::f64::consts::PI; //  3.14159265358979323846264338....

        println!("ID padded: {:05}", id);      // 00042
        println!("Value 2 decimals: {:.2}", value); // 3.14
        println!("Value width 8: {:8.2}", value);   // '    3.14'

        //ID padded: 00042
        // Value 2 decimals: 3.14
        // Value width 8:     3.14
    }


    pub fn formatting_vector_structures_to_table()
    {
        let users = vec![
            User { id: 1, name: "Alice".into(), score: 98.5 },
            User { id: 2, name: "Bob".into(), score: 76.2 },
            User { id: 3, name: "Charlie".into(), score: 88.0 },
        ];

        println!("{:<5} | {:<10} | {:>8}", "ID", "Name", "Score");
        println!("{:-<5}-+-{:-<10}-+-{:-<8}", "", "", "");

        for user in users {
            println!("{:<5} | {:<10} | {:>8.2}", user.id, user.name, user.score);
        }

        // ID    | Name       |    Score
        // ------+------------+---------
        // 1     | Alice      |    98.50
        // 2     | Bob        |    76.20
        // 3     | Charlie    |    88.00
    }


    pub fn floating_point_formatting()
    {
        let pi: f64 = std::f64::consts::PI;

        println!("{:.2}", pi);      // 3.14 (2 знака после запятой)
        println!("{:.4}", pi);      // 3.1416 (4 знака)
        println!("{:8.3}", pi);     // ширина 8, 3 знака после запятой → '   3.142'
        println!("{:0>8.3}", pi);   // паддинг нулями → '0003.142'

        // 3.14
        // 3.1416
        //    3.142
        // 0003.142
    }

    pub fn print_hex_bin()
    {
        let n: i32 = 255;

        println!("dec: {}", n);       // 255
        println!("bin: {:b}", n);     // 11111111
        println!("oct: {:o}", n);     // 377
        println!("hex: {:x}", n);     // ff
        println!("HEX: {:X}", n);     // FF

        // dec: 255
        // bin: 11111111
        // oct: 377
        // hex: ff
        // HEX: FF
    }

    pub fn named_parameters()
    {
        let name = "Alice";
        let age = 30;
        let city = "Paris";

        println!("{name:<10} | {age:>3} | {city:^10}");

        // Output:
        // Alice      |  30 |   Paris
    }

    pub fn dynamic_formatting_variable_widths()
    {
        let num: f64 = std::f64::consts::PI;
        let width: i32 = 10;
        let precision: i32 = 3;

        // width и precision задаются через позиционные или именованные параметры
        //println!("{0:>1$.*2$}", num, width, precision);
        println!("{val:>w$.p$}", val=num, w=8, p=2);
    }
}


mod print_errors
{
    pub fn example()
    {
        eprint!("Error: ");
        eprintln!("Something went wrong!");
    }
}


mod experiments
{
    use std::{thread, time::Duration, io::{self, Write}};

    pub fn dynamically_updating_string()
    {
        for i in 0..=100 {
            print!("\rProgress: {:>3}%", i);
            io::stdout().flush().unwrap(); // без flush вывод будет буфериться
            thread::sleep(Duration::from_millis(20));
        }
        println!("\nDone!");
    }
}

pub fn test_all()
{
    // pretty_table::test_all();
    logger_printer::test_all();

    // formating::width_alignment();
    // formating::edges_alignment();
    // formating::fixed_width();
    // formating::floating_point_formatting();
    // formating::print_hex_bin();
    // formating::named_parameters();
    // formating::dynamic_formatting_variable_widths();
    // formating::formatting_vector_structures_to_table();

    // experiments::dynamically_updating_string();

    // print_errors::example();
}