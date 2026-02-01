
mod simple_example
{
    struct Square {}
    struct Rectangle {}

    trait Shape {
        fn info(&self) -> &'static str;
    }

    impl Shape for Square {
        fn info(&self) -> &'static str {
            "Square"
        }
    }

    impl Shape for Rectangle {
        fn info(&self) -> &'static str {
            "Rectangle"
        }
    }

    fn print_info(shape_impl: Box<dyn Shape>)
    {
        println!("{}", shape_impl.info());
    }

    pub fn demo()
    {
        let square: Box<Square> = Box::new(Square {});
        let rectangle: Box<Rectangle> = Box::new(Rectangle {});

        print_info(square);
        print_info(rectangle);
    }
}

mod example_iterate_collection
{
    trait IDrawable {
        fn draw(&self);
    }

    struct Button {
        name: String
    }

    struct CheckBox {
        name: String
    }

    struct Label {
        name: String
    }

    impl IDrawable for Button {
        fn draw(&self) {
            println!("{}", self.name);
        }
    }

    impl IDrawable for CheckBox {
        fn draw(&self) {
            println!("{}", self.name);
        }
    }

    impl IDrawable for Label {
        fn draw(&self) {
            println!("{}", self.name);
        }
    }

    fn draw_all(items: &Vec<Box<dyn IDrawable>>)
    {
        for item in items
        {
            item.draw();
        }
    }

    pub fn demo()
    {
        let items: Vec<Box<dyn IDrawable>> = vec! {
            Box::new(Button { name: String::from( "Button") }),
            Box::new(CheckBox { name: String::from( "CheckBox") }),
            Box::new(Label { name: String::from( "Label") }),
        };

        draw_all(&items);

        // Button
        // CheckBox
        // Label
    }
}


pub fn test_all()
{
    // simple_example::demo();
    example_iterate_collection::demo();
}