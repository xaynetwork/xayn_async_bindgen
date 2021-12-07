use std::{
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
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

    #[structopt(short, long)]
    out: PathBuf,
}

fn main() {
    let cli = Cli::from_args();
    //TODO better error messages
    let file = fs::read_to_string(&cli.genesis).expect("failed to read genesis file");
    //TODO check ffi class name
    let functions = AsyncFunctionSignature::sniff_dart_signatures(&file);
    let out = BufWriter::new(File::create(&cli.out).expect("failed to create/open output file"));
    generate(&cli.ffi_class, &functions, out).expect("failed to write extension to output file");
}
