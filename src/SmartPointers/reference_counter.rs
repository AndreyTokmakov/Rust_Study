use std::rc::Rc;


fn pointer_to_vector_clone()
{
    let values: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3]);
    
    // The two syntaxes below are equivalent.
    let mut a: Rc<Vec<i32>> = values.clone();
    let mut b: Rc<Vec<i32>> = Rc::clone(&values);

    println!("values: {:?}, a: {:?}, b: {:?}", values, a, b);

    Rc::make_mut(&mut a).push(4);
    println!("values: {:?}, a: {:?}, b: {:?}", values, a, b);
    
    Rc::make_mut(&mut b).push(5);
    println!("values: {:?}, a: {:?}, b: {:?}", values, a, b);
    
    // values: [1, 2, 3], a: [1, 2, 3],    b: [1, 2, 3]
    // values: [1, 2, 3], a: [1, 2, 3, 4], b: [1, 2, 3]
    // values: [1, 2, 3], a: [1, 2, 3, 4], b: [1, 2, 3, 5]
}

fn pointer_to_vector_modify()
{
    let mut values: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3]);

    println!("values: {:?}", values);
    Rc::make_mut(&mut values).push(4);
    println!("values: {:?}", values);
}                      


// https://doc.rust-lang.org/book/ch15-04-rc.html
// Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.
pub fn test_all()
{
    pointer_to_vector_clone();
    // pointer_to_vector_modify();
}