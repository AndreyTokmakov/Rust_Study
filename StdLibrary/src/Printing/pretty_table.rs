
use prettytable::{Table, Row, Cell, row};

fn print_table()
{
    let mut table = Table::new();

    table.add_row(row!["ID", "Name", "Score"]);
    table.add_row(Row::new(vec![
        Cell::new("1"),
        Cell::new("Alice"),
        Cell::new("98.5"),
    ]));
    table.add_row(row!["2", "Bob", "76.2"]);
    table.add_row(row!["3", "Charlie", "88.0"]);

    table.printstd();

    // +----+---------+-------+
    // | ID | Name    | Score |
    // +----+---------+-------+
    // | 1  | Alice   | 98.5  |
    // +----+---------+-------+
    // | 2  | Bob     | 76.2  |
    // +----+---------+-------+
    // | 3  | Charlie | 88.0  |
    // +----+---------+-------+
}


pub fn test_all()
{
    print_table();
}