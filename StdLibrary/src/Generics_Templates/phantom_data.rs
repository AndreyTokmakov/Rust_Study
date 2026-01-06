
/**
# https://www.compilenrun.com/docs/language/rust/rust-generics/rust-phantom-data

PhantomData is a zero-sized type provided by Rust's standard library that allows you
to "mark" a data structure as if it owns or uses a particular type, even when it doesn't
actually store any data of that type.

Here's the key insight: PhantomData<T> is a marker that:
- Takes up no space at runtime (it's a zero-sized type)
- Tells the compiler that your type is conceptually associated with T
- Helps satisfy Rust's type system rules, particularly around variance and drop checking

**/

mod without_phantom_data
{   /*
    struct Identifier<T> {
        id: usize,
    }
    */

    pub fn demo()
    {
        /*
        let user_id = Identifier::<String> { id: 42 };
        println!("ID: {}", user_id.id);
        */

        // type parameter `T` is never used [E0392]
        // unused type parameter
    }
}

mod with_parameter_data
{
    use std::marker::PhantomData;

    struct Identifier<T> {
        id: usize,
        _marker: PhantomData<T>,
    }

    pub fn demo()
    {
        let user_id = Identifier::<String> { id: 42, _marker: PhantomData };
        println!("ID: {}", user_id.id);
    }
}

pub fn test_all()
{
    without_phantom_data::demo();
    with_parameter_data::demo();
}