
mod example_1
{
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

    pub fn demo()
    {
        let obj: MyStruct = MyStruct::create(1, 2);

        println!("{:?}", obj);
        println!("{:?}", MyStruct::my_static());
    }
}

/** # Function Inlining with #[inline]
The #[inline] attribute is a hint to the compiler that the body of a function should be placed
directly at the call site (inlined) instead of generating a function call instruction.
This can reduce function call overhead and enable further optimizations but should be used carefully,
as incorrect inlining can increase code size and potentially slow down the program.

The Rust compiler automatically inlines functions it deems worthwhile.
The #[inline] attributes allow you to override these heuristics.
The available attributes are:

#[inline]:         A suggestion to the compiler to inline the function.
                   It is particularly important for enabling cross-crate inlining, as the function's body needs to be available to downstream crates.
#[inline(always)]: A strong suggestion that the function should always be inlined.
                   The compiler may still ignore this hint in some cases.
#[inline(never)]:  A strong suggestion that the function should not be inlined.
                   This can be useful for debugging or reducing code size for cold (rarely called) functions, such as error handlers.


// A function the compiler will likely inline automatically
fn add_one(x: u32) -> u32 {
    x + 1
}

// Explicitly suggest inlining for a function in a library
#[inline]
pub fn public_helper(data: &str) {
    // ...
}

// Strongly suggest a function is never inlined
#[inline(never)]
fn performance_heavy_operation() {
    // ...
}

**/

pub fn test_all()
{
    example_1::demo();
}