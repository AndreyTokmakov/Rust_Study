
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

/// https://www.compilenrun.com/docs/language/rust/rust-memory-management/rust-arc-type#weak-references
mod weak_references
{
    use std::sync::{Arc, Weak};
    use std::cell::RefCell;

    struct Node
    {
        value: i32,
        parent: Option<Weak<RefCell<Node>>>,
        children: Vec<Arc<RefCell<Node>>>,
    }

    pub fn demo()
    {
        let root: Arc<RefCell<Node>> = Arc::new(RefCell::new(Node {
            value: 1,
            parent: None,
            children: vec![],
        }));

        let child: Arc<RefCell<Node>> = Arc::new(RefCell::new(Node {
            value: 2,
            parent: Some(Arc::downgrade(&root)),   // Use a Weak reference to the parent to avoid reference cycles
            children: vec![],
        }));

        // Add the child to the root's children
        root.borrow_mut().children.push(Arc::clone(&child));

        // Access the child's parent (which is a Weak reference)
        let parent: Option<Arc<RefCell<Node>>> = child.borrow().parent.as_ref().unwrap().upgrade();
        if let Some(parent) = parent {
            println!("Child's parent value: {}", parent.borrow().value);
        }

        // Output:
        // Child's parent value: 1
    }
}

// NOTE: https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety
pub fn test_all()
{
    // modify_stored_values();
    // create_mutable();
    // share_data_with_arc();

    weak_references::demo();
}