use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rust_out_dir = PathBuf::from("./src");
    let descriptor_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_path = descriptor_out_dir.join("reflection_descriptor.bin");

    // Compile .proto files
    tonic_build::configure()
        .out_dir(&rust_out_dir)
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(
            &[
                "buffers/interface/v1/interface.proto",
                "buffers/warfare/v1/warfare.proto",
                "buffers/economy/v1/economy.proto",
            ],
            &["buffers"],
        )?;

    println!("cargo:rerun-if-changed=buffers/");

    Ok(())
}
