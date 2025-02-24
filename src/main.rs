mod lex4m;
mod mlir4m;
mod node4m;
mod par4m;
mod token4m;

use clap::Parser;
use lex4m::Lex4m;
use std::path::PathBuf;

/// A funny Micro language compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets input Micro source file
    #[arg(value_name = "INPUT", default_value = "test.m")]
    source_file: PathBuf,

    /// Sets output LLVM IR file
    #[arg(short, value_name = "OUTPUT", default_value = "a.ll")]
    output_file: Option<PathBuf>,

    /// Use verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.source_file).expect("Failed to read input file");

    let mut lexer = Lex4m::new(input);
    lexer.lex();
    let tokens = lexer.tokens();

    let tokens4m = token4m::Token4m::new(tokens.clone());
    let mut parser = par4m::Par4m::new(tokens4m);
    parser.generate_concrete_syntax_tree();
    parser.generate_abstract_syntax_tree();

    let cst = parser.concrete_syntax_tree().to_dot(false);
    let ast = parser.abstract_syntax_tree().to_dot(true);

    std::fs::write("cst.dot", cst).expect("Unable to write file");
    std::fs::write("ast.dot", ast).expect("Unable to write file");

    let mut mlir = mlir4m::Mlir4m::new(parser.abstract_syntax_tree());

    let mlir_str = mlir.generate_mlir();
    // println!("{}", mlir_str);
    std::fs::write("a.mlir", mlir_str).expect("Unable to write file");
}
