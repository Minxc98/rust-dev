fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/voting.proto")?;
    println!("cargo:rerun-if-changed=../protos/voting.proto");
    Ok(())
} 