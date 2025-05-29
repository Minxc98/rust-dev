fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["../protos/voting.proto"], &["../protos"])?;
    println!("cargo:rerun-if-changed=../protos/voting.proto");
    Ok(())
} 