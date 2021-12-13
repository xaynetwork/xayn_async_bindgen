use std::{
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use gen_dart::generate;
use parse_genesis::AsyncFunctionSignature;
use structopt::StructOpt;

mod gen_dart;
mod parse_genesis;
#[cfg(test)]
mod test_utils;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generate dart code for async-bindgen")]
struct Cli {
    #[structopt(long)]
    genesis: PathBuf,

    #[structopt(long)]
    ffi_class: String,
}

fn main() {
    let cli = Cli::from_args();
    let genesis_ext = path_with_all_extensions_replaced(&cli.genesis, "ext.dart");
    let rel_path = Path::new(".").join(cli.genesis.file_name().unwrap());
    let file = fs::read_to_string(&cli.genesis).expect("failed to read genesis file");
    let module_to_functions = AsyncFunctionSignature::sniff_dart_signatures(&file);
    let mut out =
        BufWriter::new(File::create(&genesis_ext).expect("failed to create/open output file"));
    generate(&rel_path, &cli.ffi_class, &module_to_functions, &mut out)
        .expect("failed to write extension to output file");
}

fn path_with_all_extensions_replaced(path: &Path, new_extension: &str) -> PathBuf {
    let mut path = path.with_extension("");
    while path.extension().is_some() {
        path.set_extension("");
    }
    path.set_extension(new_extension);
    path
}
