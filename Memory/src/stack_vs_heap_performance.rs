use std::time::{Duration, Instant};

pub fn test_all()
{
    // Stack allocation
    let start: Instant = Instant::now();
    for _ in 0..1_000_000 {
        let _x = 42;
    }
    let stack_duration: Duration = start.elapsed();

    // Heap allocation
    let start: Instant = Instant::now();
    for _ in 0..1_000_000 {
        let _x = Box::new(42);
    }
    let heap_duration: Duration = start.elapsed();

    println!("Stack time: {:?}\nHeap time: {:?}", stack_duration, heap_duration);
    println!("Heap is approximately {}x slower",
             heap_duration.as_nanos() as f64 / stack_duration.as_nanos() as f64);
}
