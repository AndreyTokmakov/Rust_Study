
fn main() -> Result<(), Box<dyn std::error::Error>> {

    prost_build::compile_protos(&["proto/person.proto"], &["proto/"])?;
    prost_build::compile_protos(&["proto/items.proto"], &["proto/"])?;
    Ok(())
}