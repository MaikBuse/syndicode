use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the directory of the crate's Cargo.toml
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut current_dir = manifest_dir.as_path();

    let workspace_root = loop {
        // Check for a Cargo.toml file containing "[workspace]"
        if let Ok(content) = fs::read_to_string(current_dir.join("Cargo.toml")) {
            if content.contains("[workspace]") {
                break current_dir;
            }
        }

        // Move to the parent directory, or break if we've reached the root
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            // Default to the crate's directory if no workspace is found
            break manifest_dir.as_path();
        }
    };

    // Set an environment variable that the main code can use
    println!(
        "cargo:rustc-env=WORKSPACE_ROOT={}",
        workspace_root.display()
    );

    // Rerun the build script if the workspace Cargo.toml changes
    println!(
        "cargo:rerun-if-changed={}/Cargo.toml",
        workspace_root.display()
    );
}
