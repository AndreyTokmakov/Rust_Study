
trait IDrawable {
    fn draw(&self);
}

struct Button {
    label: String,
}

struct Label {
    text: String,
}

impl IDrawable for Button
{
    fn draw(&self)
    {
        println!("Button: {}", self.label);
    }
}

impl IDrawable for Label
{
    fn draw(&self)
    {
        println!("Label: {}", self.text);
    }
}

enum Widget
{
    Button(Button),
    Label(Label),
}


// Implement Drawable for the enum
// This provides static dispatch without dynamic allocation
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

fn draw_all(items: &[Widget])
{
    for item in items  {
        item.draw();
    }
}

pub fn test_all()
{
    let widgets: Vec<Widget> = vec![
        Widget::Button(Button {
            label: "OK".to_string(),
        }),
        Widget::Label(Label {
            text: "Hello".to_string(),
        }),
        Widget::Button(Button {
            label: "Cancel".to_string(),
        }),
    ];

    draw_all(&widgets);

    // Button: OK
    // Label: Hello
    // Button: Cancel
}