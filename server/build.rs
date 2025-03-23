use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let out_dir = "./src/presentation/proto";

    tonic_build::configure()
        .out_dir(out_dir)
        .file_descriptor_set_path(original_out_dir.join("reflection_descriptor.bin"))
        // .file_descriptor_set_path("src/infrastructure/proto/reflection_descriptor.bin")
        .compile_protos(
            &[
                "proto/control.proto",
                "proto/warfare.proto",
                "proto/economy.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
