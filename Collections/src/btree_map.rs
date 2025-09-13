use std::collections::BTreeMap;

fn create_and_insert()
{
    let mut map = BTreeMap::new();
    map.insert(3, "C");
    map.insert(1, "A");
    map.insert(2, "B");

    println!("{:?}", map); // {1: "A", 2: "B", 3: "C"}
}

pub fn test_all()
{
    create_and_insert();
}