

mod example_simple
{
    trait IDrawable {
        fn draw(&self);
    }

    struct Widget {
        label: String
    }

    impl IDrawable for Widget {
        fn draw(&self)
        {
            println!("Drawing button: {}", self.label);
        }
    }

    fn render(item: &impl IDrawable)
    {
        print!("Render: ");
        item.draw();
    }

    fn genericRenderer<T: IDrawable>(item: &T)
    {
        print!("GenericRenderer: ");
        item.draw();
    }

    pub fn demo()
    {
        let widget: Widget = Widget {
            label: String::from("SuperWidget"),
        };

        render(&widget);
        genericRenderer(&widget);
    }
}


mod example_one
{
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

    impl Employee {
        fn getRole(&self) -> String {
            return self.role.clone();
        }
    }

    impl Base for Employee {
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

    pub fn demo()
    {
        let employee: Employee = Base::new(1, "Bob".to_string());

        println!("employee: {:?}", employee);
        println!("role: {:?}", employee.getRole());
        employee.printInfo();

        // employee: Employee { id: 1, name: "Bob", role: "Unknown" }
        // role: "Unknown"
        // Employee info: (1 Bob)
    }
}

pub fn test_all()
{
    example_simple::demo();
    // example_one::demo();
}