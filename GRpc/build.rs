use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

fn collect_proto_files(dir: &Path, file_list: &mut Vec<PathBuf>)
{
    for entry in fs::read_dir(dir).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        if path.is_dir() {
            collect_proto_files(&path, file_list);
        } else if path.extension().and_then(|s| s.to_str()) == Some("proto") {
            file_list.push(path);
        }
    }
}

fn main()
{
    let proto_root: PathBuf = PathBuf::from("proto");
    let mut proto_files: Vec<PathBuf> = Vec::new();
    collect_proto_files(&proto_root, &mut proto_files);

    for proto in &proto_files {
        println!("Compiling proto: {}", proto.display());
        tonic_build::compile_protos(proto).unwrap();
    }
}