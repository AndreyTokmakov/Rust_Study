
mod threads;
mod mpsc_channel;
mod mutex;
mod arc;
mod atomics;
mod locking;
mod sender_receiver;

pub fn test_all()
{
    // threads::test_all();
    // atomics::test_all();
    // mpsc_channel::test_all();
    sender_receiver::test_all();
    // mutex::test_all();
    // arc::test_all();
    // locking::test_all();
}