
fn dereferencing_raw_pointer()
{
    let mut num: i32 = 5;

    let r1: *const i32 = &raw const num;
    let r2: *mut i32 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dereferencing_raw_pointer_unsafe()
{ 
    unsafe {
        let mut num: i32 = 5;
    
        let r1: *const i32 = &raw const num;
        let r2: *mut i32 = &raw mut num;
    
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}


pub fn test_all()
{
    dereferencing_raw_pointer();
    unsafe {
        dereferencing_raw_pointer_unsafe();
    }
}