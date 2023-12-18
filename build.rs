use std::io::Result;
fn main() -> Result<()> {
    let empty_imports: Vec<String> = Vec::default();
    prost_build::compile_protos(&["src/rusty.protocol.proto"], &empty_imports)?;
    Ok(())
}
