
// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Unit;


pub fn test_all()
{
    let unit = Unit;
    println!("unit = {:?}", unit);

    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;
    println!("copied_unit = {:?}, unit = {:?}", copied_unit, unit);

    // Clone `Unit`, there are no resources to move
    let cloned_unit = unit.clone();
    println!("cloned_unit = {:?}, unit = {:?}", cloned_unit, unit);
}