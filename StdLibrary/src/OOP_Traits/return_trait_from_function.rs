mod simple_example
{
    trait IDrawable {
        fn draw(&self);
    }

    struct Button;

    impl IDrawable for Button
    {
        fn draw(&self)
        {
            println!("Button");
        }
    }

    fn makeDrawable() -> impl IDrawable
    {
        Button
    }

    pub fn demo()
    {
        let obj = makeDrawable();
        obj.draw();
        // Button
    }
}

mod example_return_dyn_object
{
    trait IDrawable  {
        fn draw(&self);
    }

    struct Button;
    struct Label;

    impl IDrawable for Button
    {
        fn draw(&self)
        {
            println!("Button");
        }
    }

    impl IDrawable for Label
    {
        fn draw(&self)
        {
            println!("Label");
        }
    }

    fn makeDrawable(cond: bool) -> Box<dyn IDrawable>
    {
        if cond {
            Box::new(Button)
        }  else  {
            Box::new(Label)
        }
    }

    pub fn demo()
    {
        {
            let obj: Box<dyn IDrawable> = makeDrawable(true);
            obj.draw();
        }
        {
            let obj: Box<dyn IDrawable> = makeDrawable(false);
            obj.draw();
        }
        // Button
        // Label
    }
}

mod example_enum
{
    struct Button;
    struct Label;

    trait IDrawable  {
        fn draw(&self);
    }

    impl IDrawable for Button {
        fn draw(&self) {
            println!("Button");
        }
    }

    impl IDrawable for Label {
        fn draw(&self) {
            println!("Label");
        }
    }
    enum Widget
    {
        Button(Button),
        Label(Label),
    }

    impl IDrawable for Widget
    {
        fn draw(&self)
        {
            match self
            {
                Widget::Button(b) => b.draw(),
                Widget::Label(l) => l.draw(),
            }
        }
    }

    fn makeDrawable(cond: bool) -> Widget
    {
        if cond {
            Widget::Button(Button)
        } else {
            Widget::Label(Label)
        }
    }

    pub fn demo()
    {
        {
            let obj: Widget = makeDrawable(true);
            obj.draw();
        }
        {
            let obj: Widget = makeDrawable(false);
            obj.draw();
        }
        // Button
        // Label
    }
}

pub fn test_all()
{
    // simple_example::demo();
    // example_return_dyn_object::demo();
    example_enum::demo();
}