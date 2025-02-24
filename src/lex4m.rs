use lrlex::{lrlex_mod, LexerDef};
use lrpar::{Lexeme, Lexer, NonStreamingLexer};

lrlex_mod!("micro.l");

pub struct Lex4m {
    input: String,
    tokens: Vec<(String, String)>,
}

impl Lex4m {
    pub fn new(input: String) -> Self {
        Lex4m {
            input,
            tokens: Vec::new(),
        }
    }

    pub fn tokens(&self) -> &Vec<(String, String)> {
        &self.tokens
    }

    pub fn lex(&mut self) {
        let lexerdef = micro_l::lexerdef();
        let lexer = lexerdef.lexer(&self.input);

        for lexeme in lexer.iter() {
            match lexeme {
                Ok(lexeme) => {
                    let span = lexer.span_str(lexeme.span());
                    let tok_id = lexeme.tok_id();
                    let tok_name = lexerdef.get_rule_by_id(tok_id).name().unwrap();
                    self.tokens.push((tok_name.to_string(), span.to_string()));
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }

        self.tokens.push(("SCANEOF".to_string(), "".to_string()));
    }
}
