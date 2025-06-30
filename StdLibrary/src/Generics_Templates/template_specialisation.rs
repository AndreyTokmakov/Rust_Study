
struct ObjectEx<T> {
    value: T
}

impl ObjectEx<i32> {
    fn getValue(&self) -> &i32 {
        &self.value
    }
}

impl ObjectEx<String> {
    fn getValue(&self) -> String  {
        String::from("String_Value!!!!")
    }
}

fn template_specialization()
{
    {
        let v: ObjectEx<i32> = ObjectEx { value: 11223 };
        let val = v.getValue();
        println!("{}", val);
    }
    {
        let v: ObjectEx<String> = ObjectEx { value: String::from("") };
        let val = v.getValue();
        println!("{}", val);
    }
}

pub fn test_all()
{
    template_specialization();
}