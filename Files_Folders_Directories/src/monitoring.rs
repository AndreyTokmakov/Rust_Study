

mod inotify_example
{
    use inotify::{Inotify, WatchMask};
    use std::error::Error;

    pub fn run() -> Result<(), Box<dyn Error>>
    {
        let path: String = String::from("/home/andtokm/DiskS/Temp/monitoring");
        let mut inotify: Inotify = Inotify::init()?;
        inotify.add_watch(
            path,
            WatchMask::CREATE | WatchMask::MODIFY | WatchMask::DELETE,
        )?;

        let mut buffer = [0u8; 4096];

        loop {
            let events = inotify.read_events_blocking(&mut buffer)?;
            for event in events  {
                println!("Event: {:?}", event);
            }
        }
    }
}

mod notify_example
{
    use std::path::Path;
    use notify::{Watcher, RecursiveMode, Result};
    use notify::recommended_watcher;
    use std::sync::mpsc::channel;

    fn main() -> Result<()>
    {
        let path: &Path = Path::new("/home/andtokm/DiskS/Temp/monitoring");
        let (tx, rx) = channel();
        let mut watcher = recommended_watcher(tx)?;
        watcher.watch(path, RecursiveMode::Recursive)?;
        println!("Watching...");

        for res in rx {
            match res {
                Ok(event) => println!("Event: {:?}", event),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        Ok(())
    }
}

pub fn test_all()
{
    inotify_example::run().expect("TODO: panic message");
}