fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/records.proto")?;
    tonic_build::compile_protos("proto/hero.proto")?;
    Ok(())
}
