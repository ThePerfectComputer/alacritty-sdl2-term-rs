use std::env;
use std::path::PathBuf;

fn main() {
    pkg_config::probe_library("sdl2").unwrap();
}