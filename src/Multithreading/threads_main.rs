
mod threads;
mod channels;
mod mutex;
mod arc;
mod atomics;
mod locking;

pub fn test_all()
{
    // threads::test_all();
    // atomics::test_all();
    channels::test_all();
    // mutex::test_all();
    // arc::test_all();
    // locking::test_all();
}