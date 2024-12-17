use std::env;
use std::path::PathBuf;

fn main() {
    pkg_config::probe_library("sdl2").unwrap();

    // Determine the correct include path based on the architecture
    let include_path = if cfg!(target_arch = "aarch64") {
        // Apple Silicon Macs
        "-I/opt/homebrew/include"
    } else {
        // Intel Macs
        "-I/usr/local/include"
    };

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
}