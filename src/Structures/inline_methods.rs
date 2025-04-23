
#[derive(Debug)]
struct MyStruct {
    x: i32,
    y: i32,
}

impl MyStruct
{
    
    fn create(x: i32, y: i32) -> Self {
        return Self { x, y }
    }
    
    #[inline]
    pub fn my_static() -> i32 {
        123
    }
}

pub fn test_all()
{
    let obj: MyStruct = MyStruct::create(1, 2);
    
    println!("{:?}", obj);
    println!("{:?}", MyStruct::my_static());
}