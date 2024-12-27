use List::*;

enum List
{
    // Cons: A tuple structure that stores an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: Node indicating the end of the linked list
    Nil,
}

// Methods can be attached to an enumeration
impl List {
    // Creating an empty list
    fn new() -> List {
        // `Nil` is of type `List`
        Nil
    }

    // A function that takes a list and returns the same list, but with a new element at the beginning
    fn prepend(self, elem: u32) -> List {
        // `Cons` is also of type `List`
        Cons(elem, Box::new(self))
    }

    // Returning the length of the list
    fn len(&self) -> u32 {
        // `self` must be matched (checked for compliance),
        // since the behavior of this method depends on the `self` option
        // `self` is of type `&List', and `*self` is of type `List', mapping to
        // the specific type `T` is preferable than matching by reference `&T`
        match *self {
            // We can't get hold of `tail` because `self` is borrowed; instead, we'll take a reference to `tail`
            Cons(_, ref tail) => 1 + tail.len(),
            // Base case: An empty list has zero length
            Nil => 0
        }
    }

    // We return the list representation in the form of a string (placed in the heap)
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a string placed in the heap, instead of output to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}


pub fn test_all()
{
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("List size: {}", list.len());
    println!("{}", list.stringify());
}