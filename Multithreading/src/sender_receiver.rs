// #![feature(mpmc_channel)]


mod simple_example
{
    use std::sync::mpsc;
    use std::thread;
    
    pub fn demo()
    {
        let (sender, receiver) = mpsc::channel();
        let sender2 = sender.clone();

        // First thread owns sender
        thread::spawn(move || {
            sender.send(1).unwrap();
        });

        // Second thread owns sender2
        thread::spawn(move || {
            sender2.send(2).unwrap();
        });

        let msg: i32  = receiver.recv().unwrap();
        let msg2: i32 = receiver.recv().unwrap();

        assert_eq!(3, msg + msg2);
    }
}



pub fn test_all()
{
    simple_example::demo();
}