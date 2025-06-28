

/*
mod basic
{
    use std::thread;
    use std::sync::mpmc::channel;

    pub fn demo()
    {
        // Create a simple streaming channel
        let (tx, rx) = channel();
        thread::spawn(move || {
            tx.send(10).unwrap();
        });
        assert_eq!(rx.recv().unwrap(), 10);
    }
}
*/

pub fn test_all()
{

}