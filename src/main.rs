use clap::Parser;
use std::path::PathBuf;

/// A funny Micro language compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets input Micro source file
    #[arg(value_name = "INPUT")]
    source_file: PathBuf,

    /// Sets output LLVM IR file
    #[arg(short, value_name = "OUTPUT", default_value = "a.ll")]
    output_file: Option<PathBuf>,

    /// Use verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let _args = Args::parse();
}
