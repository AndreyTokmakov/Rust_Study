
mod allocator_tests
{
    /*
    use std::alloc::{
        GlobalAlloc,
        Layout,
        System
    };

    struct MyAllocator;

    unsafe impl GlobalAlloc for MyAllocator
    {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            println!("Allocating {} bytes", layout.size());
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
        {
            println!("Deallocating {} bytes", layout.size());
            System.dealloc(ptr, layout);
        }
    }

    #[global_allocator]
    static ALLOCATOR: MyAllocator = MyAllocator;

    pub fn demo()
    {
        let boxed_value: Box<i32> = Box::new(1000);
        println!("Value: {}", *boxed_value);
        // boxed_value is deallocated when it goes out of scope
    }
    */
}

// INFO: https://www.compilenrun.com/docs/language/rust/rust-memory-management/
//    rust-box-type#practical-example-custom-memory-allocation
pub fn test_all()
{
    // allocator_tests::demo();
}