pub struct Token4m {
    tokens: Vec<(String, String)>,
    current_index: usize,
}

impl Token4m {
    pub fn new(tokens: Vec<(String, String)>) -> Self {
        Token4m {
            tokens,
            current_index: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<(String, String)> {
        if self.current_index < self.tokens.len() {
            let token = self.tokens[self.current_index].clone();
            Some(token)
        } else {
            None
        }
    }

    pub fn consume_token(&mut self) {
        self.current_index += 1;
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}
