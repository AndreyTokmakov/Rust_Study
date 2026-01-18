
#[derive(Debug)]
struct Employee
{
    id: u32,
    name: String,
    role: String,
}

trait Base
{
    fn new(id: u32, name: String) -> Self;

    fn getId(&self) -> u32;

    fn getName(&self) -> String;

    fn printInfo(&self) {
        println!("Base info: ({} {})", self.getId(), self.getName());
    }
}

impl Employee
{
    fn getRole(&self) -> String {
        return self.role.clone();
    }
}

impl Base for Employee
{
    fn new(id: u32, name: String) -> Employee {
        return Employee { id, name,  role: String::from("Unknown") };
    }

    fn getId(&self) -> u32 {
        self.id
    }

    fn getName(&self) -> String {
        return self.name.clone();
    }

    // This will overwrite Base::printInfo()
    fn printInfo(&self) {
        println!("Employee info: ({} {})", self.getId(), self.getName());
    }
}

pub fn test_all()
{
    let employee: Employee = Base::new(1, "Bob".to_string());
    
    println!("employee: {:?}", employee);
    println!("role: {:?}", employee.getRole());
    employee.printInfo();
}