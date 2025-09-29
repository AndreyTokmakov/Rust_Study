
pub trait Migration
{
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}

pub struct CreateTable;
pub struct AddField;
struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Migration for CreateTable
{
    fn execute(&self) -> &str {
        "create table"
    }
    fn rollback(&self) -> &str {
        "drop table"
    }
}

impl Migration for AddField
{
    fn execute(&self) -> &str {
        "add field"
    }
    fn rollback(&self) -> &str {
        "remove field"
    }
}

impl Schema
{
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev() // reverse iterator's direction
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

// https://rust-unofficial.github.io/patterns/patterns/behavioural/command.html
pub fn test_all()
{
    let mut schema: Schema = Schema::new();

    let cmd: Box<CreateTable> = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
