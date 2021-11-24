use config::Config;
use interface::InterfaceDescription;

mod config;
mod interface;
mod dart;
mod gen_rust;
#[cfg(test)]
mod test_utils;

pub fn generate_bindings(config: Config) -> Result<(), anyhow::Error> {
    todo!();
    // let ifd = InterfaceDescription::parse(&config)?;
    // gen_rust::generate(&ifd, &config)?;
    // //TODO gen dart
    // Ok(())
}

/*
 generate rust ffi
 cbindgen (rust ffi -> c header)
 ffigen (c header -> dart gen)
 grap_dart_defs (gart gen -> function signatures)
 gen dart wrapper
*/

//  (\s*///.*$)*
//  (ty)

#[derive(Clone, Copy)]
pub enum Language {
    Dart
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Dart => "dart",
        }
    }
}