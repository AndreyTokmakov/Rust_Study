use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

fn ensure_dir(path: &PathBuf)
{
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}

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

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let out_dir: PathBuf = PathBuf::from("src/generated");
    let proto_root: PathBuf = PathBuf::from("proto");
    let mut proto_files: Vec<PathBuf> = Vec::new();

    collect_proto_files(&proto_root, &mut proto_files);
    ensure_dir(&out_dir);

    for proto in &proto_files {
        println!("cargo:warning=Compiling proto: {} -> {}", proto.display(), out_dir.display());
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    let mut config: prost_build::Config = prost_build::Config::new();
    config.out_dir(&out_dir);

    config.compile_protos(&proto_files, &[proto_root] )?;
    Ok(())
}