use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(&["proto/google/main.proto"], &["proto/google"])?;

    Ok(())
}
