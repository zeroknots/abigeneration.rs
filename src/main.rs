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
    rust_file_generation(&args.abi_file, &args.out_file)?;
    Ok(())
}

fn rust_file_generation(abi_source: &str, out_file: &str) -> Result<()> {
    let _ = out_file;
    let abi_name = abi_source
        .split("/")
        .last()
        .unwrap()
        .split(".")
        .next()
        .unwrap();
    // let out_file = std::env::current_dir()?.join("ierc20.rs");
    let out_file = std::env::current_dir()?.join(abi_name).with_extension("rs");
    println!("abi_name: {}", abi_name);
    println!("abi_source: {}", abi_source);
    println!("out_file: {}", out_file.display());
    if out_file.exists() {
        std::fs::remove_file(&out_file)?;
    }

    Abigen::new(abi_name, abi_source)?
        .generate()?
        .write_to_file(out_file)?;
    Ok(())
}
