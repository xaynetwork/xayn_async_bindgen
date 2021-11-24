use std::path::PathBuf;

pub struct Config {
    //TODO builder
    pub interface_description: PathBuf,
    pub rust_out: PathBuf,
    pub dart_out: PathBuf,
    pub emit_cargo_hints: bool,
}