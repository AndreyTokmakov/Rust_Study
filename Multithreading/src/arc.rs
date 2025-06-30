
use std::sync::Arc;

trait Printable
{
    fn print(&self);
}

struct MyData 
{
    data: String,
}

impl Printable for MyData 
{
    fn print(&self) {
        println!("MyData: {}", self.data);
    }
}

fn modify_stored_values()
{
    let mut ptrOne: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3, 4, 5]);
    let ptrTwo: Arc<Vec<i32>> = Arc::clone(&ptrOne);
    
    println!("one: {:?} two: {:?}", ptrOne, ptrTwo);

    Arc::make_mut(&mut ptrOne).pop();

    println!("one: {:?} two: {:?}", ptrOne, ptrTwo);
}

fn create_mutable()
{
    let mut data: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);

    // This will clone the vector only if there are other references to it
    Arc::make_mut(&mut data).push(4);

    assert_eq!(*data, vec![1, 2, 3, 4]);
}

fn share_data_with_arc()
{
    let my_data = MyData { data: "Hello, Arc!".to_string() };
    
    let arc_my_data: Arc<dyn Printable> = Arc::new(my_data);
    let cloned_arc_my_data = Arc::clone(&arc_my_data);

    arc_my_data.print();
    cloned_arc_my_data.print();
}

// NOTE: https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety
pub fn test_all()
{
    modify_stored_values();
    // create_mutable();

    // share_data_with_arc();
}