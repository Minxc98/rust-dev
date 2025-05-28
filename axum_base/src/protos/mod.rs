pub mod voting {
    tonic::include_proto!("voting");
}

#[test]
fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/voting.proto")?;
    Ok(())
}

