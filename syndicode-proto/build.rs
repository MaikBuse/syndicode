use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rust_out_dir = "./src";
    let descriptor_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_path = descriptor_out_dir.join("reflection_descriptor.bin");

    tonic_build::configure()
        .out_dir(rust_out_dir)
        .file_descriptor_set_path(descriptor_path) // Reflection descriptor goes to OUT_DIR
        .compile_protos(
            &[
                "buffers/control.proto",
                "buffers/warfare.proto",
                "buffers/economy.proto",
            ],
            &["buffers"], // <- this is the key fix!
        )?;

    println!("cargo:rerun-if-changed=buffers/");
    Ok(())
}
