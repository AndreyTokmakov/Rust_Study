
mod async_file_monitor
{
    use std::env;
    use notify::{RecommendedWatcher, RecursiveMode, Result, Event, Config, Watcher, INotifyWatcher};
    use tokio::sync::mpsc;
    use std::path::PathBuf;
    use tokio::sync::mpsc::Sender;

    #[tokio::main]
    pub async fn run() -> Result<()> {
        let (tx, mut rx) = mpsc::channel(100);

        // Создаём watcher с callback, который отправляет события в канал
        let mut watcher: INotifyWatcher = RecommendedWatcher::new(
            move |res| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    match res {
                        Ok(event) => {
                            if tx.send(event).await.is_err() {
                                eprintln!("Receiver dropped");
                            }
                        }
                        Err(e) => eprintln!("Watch error: {e:?}"),
                    }
                });
            },
            Config::default(),
        )?;

        let path: PathBuf = env::current_dir()?.join("resources/test_files/in_out.txt");
        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        println!("Watching {:?} for changes...", path);
        while let Some(event) = rx.recv().await {
            println!("📝 File changed: {event:?}");
        }

        Ok(())
    }
}


pub fn test_all()
{
    async_file_monitor::run();
}