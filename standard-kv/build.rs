use std::io::Result;

// https://docs.rs/prost-build/latest/prost_build/
fn main() -> Result<()> {
    prost_build::Config::new()
        .bytes(&["."])
        .type_attribute(".", "#[derive(PartialOrd)]")
        .out_dir("src")
        .compile_protos(&["./proto/abi.proto"], &["proto/"])?;
    Ok(())
}
