use std::path::PathBuf;

use clap::Parser;
use ethers::prelude::Abigen;
use eyre::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long = "file", short = 'f')]
    abi_file: String,
    #[arg(long = "out", short = 'o')]
    out_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let out_file = &std::env::current_dir()?
        .join(args.out_file);
    // check if file extension is rs. if not add rs
    let out_file = if out_file.extension().is_none() {
        out_file.with_extension("rs")
    } else {
        out_file.to_path_buf()
    };
    rust_file_generation(&args.abi_file, out_file)?;
    Ok(())
}

fn rust_file_generation(abi_source: &str, out_file: PathBuf) -> Result<()> {
    let abi_name = abi_source
        .split("/")
        .last()
        .unwrap()
        .split(".")
        .next()
        .unwrap();
    println!("Wrote File to: {:?}", out_file);
    if out_file.exists() {
        std::fs::remove_file(&out_file)?;
    }

    Abigen::new(abi_name, abi_source)?
        .generate()?
        .write_to_file(out_file)?;
    Ok(())
}
