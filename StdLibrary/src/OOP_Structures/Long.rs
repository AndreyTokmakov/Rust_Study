
#[derive(Debug)]
struct Long
{
    value: i64
}

impl Long
{
    fn origin() -> Long
    {
        Long { value: 0}
    }

    // Another static method, takes two arguments
    fn create(v: i64) -> Self
    {
        Self { value: v }
    }

    fn getValue(&self) -> i64
    {
        return self.value;
    }
}


fn test1()
{
    let l1: Long = Long::origin();
    println!("{:?}", l1);

    let l2: Long = Long::create(123);
    println!("{:?}", l2);
}

pub fn tests()
{
    test1();
}