
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

        let path: PathBuf = env::current_dir()?.join("../resources/test_files/in_out.txt");
        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        println!("Watching {:?} for changes...", path);
        while let Some(event) = rx.recv().await {
            println!("📝 File changed: {event:?}");
        }

        Ok(())
    }
}

mod async_file_read
{
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;
    use std::path::{PathBuf};
    use std::env;

    #[tokio::main]
    async fn readFile() -> Result<(), Box<dyn std::error::Error>>
    {
        let file_path: PathBuf = env::current_dir()?.join("resources/test_files/file1.txt");

        let mut file: File = File::open(file_path).await?;
        let mut content: String = String::new();
        file.read_to_string(&mut content).await?;
        println!("{}", content);
        Ok(())
    }

    pub fn run()
    {
        let _ = readFile();
    }
}


pub fn test_all()
{
    // let _ = async_file_monitor::run();
    async_file_read::run();
}