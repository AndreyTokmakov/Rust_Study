
mod basic
{
    use std::sync::Arc;
    use std::thread;

    pub fn example()
    {
        let arcVec: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);

        for _ in 0..3 {
            let aClone: Arc<Vec<i32>> = Arc::clone(&arcVec);
            thread::spawn(move || {
                println!("{:?}", aClone);
            });
        }
    }
}


/**
    Arc<T> — потокобезопасный Rc
    Arc — Atomic Reference Counted — для многопоточности.
**/
pub fn test_all()
{
    basic::example();

}