use lrlex::CTLexerBuilder;

fn main() {
    // Build the lexer
    CTLexerBuilder::new()
        .lexer_in_src_dir("micro.l")
        .unwrap()
        .build()
        .unwrap();
}
