
use std::fmt::Debug;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{ Cons, Nil };


fn simple_example()
{
    let boxed_int: Box<i32> = Box::new(5);
    println!("boxed_int = {boxed_int}");
}

fn example1_basic()
{
    let x: i32 = 5;                   // Обычная переменная на стеке
    let y: Box<i32> = Box::new(5); // Данные на куче через Box

    println!("x = {}, y = {}", x, y);
    assert_eq!(*y, 5); // Разыменование через *
}
fn pointer_as_Reference()
{
    let stack_var: i32 = 5;
    let ref_var: &i32 = &stack_var;
    let ptr: Box<i32> = Box::new(stack_var);

    assert_eq!(5, stack_var);
    assert_eq!(5, *ref_var);
    assert_eq!(5, *ptr);
}


fn recursive_list()
{
    // Создаём список: 1 -> 2 -> 3 -> Nil
    let list: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    // Функция для подсчёта длины
    fn list_length<T>(list: &List<T>) -> u32 {
        match list {
            Cons(_, tail) => 1 + list_length(tail),
            Nil => 0,
        }
    }
    println!("Length: {}", list_length(&list));
}

trait Sound {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;
struct Cow;

impl Sound for Dog { fn make_sound(&self) -> String { "Woof!".to_string() } }
impl Sound for Cat { fn make_sound(&self) -> String {  "Meow!".to_string() } }
impl Sound for Cow { fn make_sound(&self) -> String { "Moo!".to_string() } }

fn trait_objects__dynamic_dispatch()
{
    // Вектор из разных типов, реализующих Sound
    let animals: Vec<Box<dyn Sound>> = vec![
        Box::new(Dog), Box::new(Cat),  Box::new(Cow),
    ];
    for animal in animals {
        println!("{}", animal.make_sound());
    }
}

mod data_transfer
{
    #[derive(Debug, Clone)]
    struct LargeData
    {
        buffer: [u8; 1000],
        metadata: String,
    }

    fn process_data(data: Box<LargeData>)
    {
        println!("Processing: {}", data.metadata);
        // data удаляется здесь, память освобождается
    }

    pub fn move_large_data()
    {
        let data: Box<LargeData> = Box::new(LargeData {
            buffer: [0; 1000],
            metadata: "Important data".to_string(),
        });
        process_data(data);
    }
}

mod type_state_pattern
{
    struct DraftPost {
        content: String,
    }

    struct PendingReview {
        content: String,
    }

    struct PublishedPost {
        content: String,
    }

    impl DraftPost
    {
        fn new() -> Self {
            DraftPost {
                content: String::new(),
            }
        }

        fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        fn request_review(self) -> PendingReview {
            PendingReview {
                content: self.content,
            }
        }
    }

    impl PendingReview
    {
        fn approve(self) -> PublishedPost {
            PublishedPost {
                content: self.content,
            }
        }

        fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }

    impl PublishedPost {
        fn content(&self) -> &str {
            &self.content
        }
    }

    pub fn demo()
    {
        let mut draft: Box<DraftPost> = Box::new(DraftPost::new());
        draft.add_text("Hello, world!");

        let pending: PendingReview = draft.request_review();
        let published: PublishedPost = pending.approve();

        println!("Published: {}", published.content());

        // Нельзя вызвать add_text на опубликованном посте — ошибка компиляции!
        // published.add_text("new text"); // ❌ не скомпилируется
    }
}

mod custom_smart_pointer
{
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("MyBox is being dropped!");
        }
    }

    pub fn demo()
    {
        let x: i32  = 5;
        let y: MyBox<i32> = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // Работает благодаря Deref
        // Drop вызывается автоматически при выходе из области видимости
    }
}

// INFO: https://www.compilenrun.com/docs/language/rust/rust-memory-management/rust-box-type
pub fn test_all()
{
    // simple_example();
    // example1_basic();
    // recursive_list();
    // trait_objects__dynamic_dispatch();
    // data_transfer::move_large_data();
    // type_state_pattern::demo();
    custom_smart_pointer::demo();
    // pointer_as_Reference();
}