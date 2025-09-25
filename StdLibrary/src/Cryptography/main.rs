

use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA256};

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader: BufReader<File> = BufReader::new(File::open(&filepath)?);
    let mut context: Context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count: usize = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

// This example calculates the SHA256 for every file with iso extension in the current directory.
// A threadpool generates threads equal to the number of cores present in the system found with num_cpus::get.
//  Walkdir::new iterates the current directory and calls execute to perform the operations of reading and computing SHA256 hash.

fn calc_SHA_for_dir() -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    for entry in WalkDir::new("/home/andtokm/DiskS/Temp/Folder_For_Testing/dst_dir2")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() /*&& is_iso(e.path()) */ ) {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);
            tx.send(digest).expect("Could not send data!");
        });
    }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}

pub fn test_all()
{

}