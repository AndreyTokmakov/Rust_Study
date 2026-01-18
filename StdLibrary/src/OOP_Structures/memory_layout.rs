
mod example_1
{
    struct Person {
        name: String,           // 24 bytes
        age: u32,               // 4 bytes
        is_employed: bool,      // 1 + padding (3 bytes)
    }

    pub fn test_size()
    {
        let employee = Person {
            name: String::from("Alice"),
            age: 30,
            is_employed: true,
        };

        println!("Size of Person: {} bytes", std::mem::size_of::<Person>());
        // Size of Person: 32 bytes
    }
}

mod controlling_memory_layout
{
    // By default, Rust doesn't guarantee field order for optimization reasons.
    // If you need a specific memory layout (e.g., for FFI with C code), you can use the #[repr(C)] attribute:
    #[repr(C)]
    struct CCompatible
    {
        a: u8,
        b: u64,
        c: u16,
    }

    //  to eliminate padding completely (at the cost of performance), you can use #[repr(packed)]:
    #[repr(packed)]
    struct PackedData
    {
        a: u8,
        b: u64,
        c: u16,
    }

    pub fn demo()
    {
        println!("CCompatible:\n\tSize: {} bytes", std::mem::size_of::<CCompatible>());
        println!("\tAlignment: {}", std::mem::align_of::<CCompatible>());

        println!("PackedData:\n\tSize: {} bytes", std::mem::size_of::<PackedData>());
        println!("\tAlignment: {}", std::mem::align_of::<PackedData>());

        // Size of CCompatible: 24 bytes
        // Size of PackedData : 11 bytes
    }
}

pub fn test_all()
{
    // example_1::test_size();
    controlling_memory_layout::demo();
}