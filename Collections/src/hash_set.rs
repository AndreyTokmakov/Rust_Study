use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use url::Host;

fn info<T: fmt::Debug>(set: &HashSet<T>)
{
    println!("Data     {:?}", set);
    println!("len      {:}", set.len());
    println!("capacity {:}", set.capacity());
    println!("empty    {:}", set.is_empty());
}

fn create()
{
    {
        let set: HashSet<i32> = HashSet::new();
        info(&set);
    }
    {
        let set = HashSet::from([1, 2, 3]);
        info(&set);
    }
}

fn create_str()
{
    let set: HashSet<&'static str> = HashSet::from(["a", "b", "c"]);
    info(&set);
}

fn insert()
{
    let mut numbers: HashSet<i32> = HashSet::new();
    numbers.insert(1);
    info(&numbers);
}

fn remove()
{
    let mut numbers: HashSet<i32> = HashSet::from([1, 2, 3]);
    info(&numbers);

    numbers.remove(&2);
    info(&numbers);
}


fn contains()
{
    let mut numbers: HashSet<i32> = HashSet::from([1, 2, 3]);

    let value: i32 = 2;

    if numbers.contains(&2) {
        println!("Number {} present", &value);
    }
    else {
        println!("Number {} missing", &value);
    }

    numbers.remove(&value);

    if numbers.contains(&2) {
        println!("Number {} present", &value);
    }
    else {
        println!("Number {} missing", &value);
    }
}


fn union()
{
    let numbers1: HashSet<i32> = HashSet::from([1, 2, 3]);
    let numbers2: HashSet<i32> = HashSet::from([2, 3, 4]);

    let numbers3: HashSet<i32> = numbers1.union(&numbers2).cloned().collect();
    println!("{:?} + {:?} ---> {:?}", &numbers1, &numbers2, &numbers3);
}



pub fn test_all()
{
    // create();
    // create_str();
    // insert();
    // remove();
    // contains();
    union();
}