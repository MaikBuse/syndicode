use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rust_out_dir = "./src/presentation/proto";
    let descriptor_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_path = descriptor_out_dir.join("reflection_descriptor.bin");

    tonic_build::configure()
        .out_dir(rust_out_dir) // Generated .rs files go here
        .file_descriptor_set_path(descriptor_path) // Reflection descriptor goes to OUT_DIR
        .compile_protos(
            &[
                "proto/control.proto",
                "proto/warfare.proto",
                "proto/economy.proto",
            ],
            &["proto"],
        )?;

    println!("cargo:rerun-if-changed=proto/");
    Ok(())
}
