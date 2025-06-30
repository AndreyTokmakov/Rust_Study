
static mut ERROR: isize = 0;

pub fn test_all()
{
    unsafe {
        ERROR += 1;
    }
    
    println!("{}", unsafe { ERROR });
    
}