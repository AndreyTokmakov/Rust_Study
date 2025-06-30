
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn demo_1()
{
    let immutable_box: Box<u32> = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box: Box<u32> = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}


fn demo_2()
{
    // Create an immutable Book named `const_book`
    let const_book = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `const_book` and call it `mutable_book`
    let mut mutable_book = const_book;

    // Immutably borrow an immutable object
    borrow_book(&const_book);

    // Immutably borrow a mutable object
    borrow_book(&mutable_book);

    // Borrow a mutable object as mutable
    new_edition(&mut mutable_book);

    // Error! Cannot borrow an immutable object as mutable
    // new_edition(&mut const_book);
    // FIXME ^ Comment out this line
}


pub fn test_all()
{
    // demo_1();
    demo_2();
}