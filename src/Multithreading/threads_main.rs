
mod threads;
mod channels;
mod mutex;
mod arc;

pub fn test_all()
{
    threads::test_all();
    // channels::test_all();
    // mutex::test_all();
    // arc::test_all();
}