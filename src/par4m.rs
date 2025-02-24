use crate::node4m::Node;
use crate::token4m::Token4m;

pub struct Par4m {
    tokens: Token4m,
    concrete_syntax_tree: Node,
    abstract_syntax_tree: Node,
}

impl Par4m {
    pub fn concrete_syntax_tree(&self) -> &Node {
        &self.concrete_syntax_tree
    }

    pub fn abstract_syntax_tree(&self) -> &Node {
        &self.abstract_syntax_tree
    }

    pub fn new(tokens: Token4m) -> Self {
        Par4m {
            tokens,
            concrete_syntax_tree: Node::new("ConcreteSyntaxTree".to_string(), "".to_string()),
            abstract_syntax_tree: Node::new("AbstractSyntaxTree".to_string(), "".to_string()),
        }
    }

    /*
    AST
     */

    pub fn generate_abstract_syntax_tree(&mut self) {
        let mut root_node = Node::new(
            "AbstractSyntaxTree".to_string(),
            "AbstractSyntaxTree".to_string(),
        );
        self.tokens.reset();
        crate::node4m::reset_global_counter();

        self._ast_start(&mut root_node);

        self.abstract_syntax_tree = root_node;
    }

    // <start> ::= <program> SCANEOF
    fn _ast_start(&mut self, father_node: &mut Node) {
        self._ast_program(father_node);

        if let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "SCANEOF" {
                self.tokens.consume_token();
            }
        }
    }

    // <program> ::= BEGIN <statement_list> END
    fn _ast_program(&mut self, father_node: &mut Node) {
        if let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "BEGIN" {
                self.tokens.consume_token();
            }
        }

        self._ast_statement_list(father_node);

        if let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "END" {
                self.tokens.consume_token();
            }
        }
    }

    // <statement_list> ::= <statement> { <statement> }
    fn _ast_statement_list(&mut self, father_node: &mut Node) {
        let mut statement_list_node = Node::new(
            "<statement list>".to_string(),
            "<statement list>".to_string(),
        );

        self._ast_statement(&mut statement_list_node);
        while let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "ID"
                || token_type.as_str() == "READ"
                || token_type.as_str() == "WRITE"
            {
                self._ast_statement(&mut statement_list_node);
            } else {
                break;
            }
        }

        father_node.add_child(statement_list_node);
    }

    // <statement> ::= ID ASSIGNOP <expression> SEMICOLON
    //              | READ LPAREN <id_list> RPAREN SEMICOLON
    //              | WRITE LPAREN <expression_list> RPAREN SEMICOLON
    fn _ast_statement(&mut self, father_node: &mut Node) {
        let (token_type, _) = self.tokens.next_token().unwrap();

        match token_type.as_str() {
            "ID" => self._ast_statement_id(father_node),
            "READ" => self._ast_statement_read(father_node),
            "WRITE" => self._ast_statement_write(father_node),
            _ => (),
        }
    }

    // <statement> ::= ID ASSIGNOP <expression> SEMICOLON
    fn _ast_statement_id(&mut self, father_node: &mut Node) {
        let mut assign_op_node = Node::new("ASSIGNOP".to_string(), ":=".to_string());

        let (_, token_value) = self.tokens.next_token().unwrap();
        assign_op_node.add_child(Node::new("ID".to_string(), token_value));
        self.tokens.consume_token();

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "ASSIGNOP" {
            self.tokens.consume_token();
        }

        self._ast_expression(&mut assign_op_node);

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "SEMICOLON" {
            self.tokens.consume_token();
        }

        father_node.add_child(assign_op_node);
    }

    // <statement> ::= READ LPAREN <id_list> RPAREN SEMICOLON
    fn _ast_statement_read(&mut self, father_node: &mut Node) {
        let mut read_node = Node::new("READ".to_string(), "read".to_string());
        self.tokens.consume_token();

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "LPAREN" {
            self.tokens.consume_token();
        }

        self._ast_id_list(&mut read_node);

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "RPAREN" {
            self.tokens.consume_token();
        }

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "SEMICOLON" {
            self.tokens.consume_token();
        }

        father_node.add_child(read_node);
    }

    // <statement> ::= WRITE LPAREN <expression_list> RPAREN SEMICOLON
    fn _ast_statement_write(&mut self, father_node: &mut Node) {
        let mut write_node = Node::new("WRITE".to_string(), "write".to_string());
        self.tokens.consume_token();

        self.tokens.consume_token();

        self._ast_expression_list(&mut write_node);

        self.tokens.consume_token();

        self.tokens.consume_token();

        father_node.add_child(write_node);
    }

    // <id_list> ::= ID { COMMA ID }
    fn _ast_id_list(&mut self, father_node: &mut Node) {
        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "ID" {
            father_node.add_child(Node::new("ID".to_string(), token_value));
            self.tokens.consume_token();
        }

        while let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "COMMA" {
                self.tokens.consume_token();
                let (token_type, token_value) = self.tokens.next_token().unwrap();
                if token_type.as_str() == "ID" {
                    father_node.add_child(Node::new("ID".to_string(), token_value));
                    self.tokens.consume_token();
                }
            } else {
                break;
            }
        }
    }

    // <expression_list> ::= <expression> { COMMA <expression> }
    fn _ast_expression_list(&mut self, father_node: &mut Node) {
        self._ast_expression(father_node);

        while let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "COMMA" {
                self.tokens.consume_token();
                self._ast_expression(father_node);
            } else {
                break;
            }
        }
    }

    // <expression> ::= <primary> { <add_op> <primary> }
    // CHANGE TO: <expression> ::= <primary> { <add_op> <expression> }
    fn _ast_expression(&mut self, father_node: &mut Node) {
        let mut add_op_node = Node::new("TMP".to_string(), "TMP".to_string());

        self._ast_primary(&mut add_op_node);

        let mut flag = false;
        while let Some(token) = self.tokens.next_token() {
            let (token_type, token_value) = token;
            if token_type.as_str() == "PLUSOP" || token_type.as_str() == "MINUSOP" {
                flag = true;

                add_op_node.set_name(token_type);
                add_op_node.set_value(token_value);

                self.tokens.consume_token();

                self._ast_expression(&mut add_op_node);
            } else {
                break;
            }
        }

        if flag {
            father_node.add_child(add_op_node);
        } else {
            let children = add_op_node.take_children();
            for child in children {
                father_node.add_child(child);
            }
        }
    }

    // <primary> ::= INTLITERAL | ID | LPAREN <expression> RPAREN
    fn _ast_primary(&mut self, father_node: &mut Node) {
        let (token_type, _) = self.tokens.next_token().unwrap();

        match token_type.as_str() {
            "INTLITERAL" => self._ast_primary_intliteral(father_node),
            "ID" => self._ast_primary_id(father_node),
            "LPAREN" => self._ast_primary_paren(father_node),
            _ => (),
        }
    }

    // <primary> ::= INTLITERAL
    fn _ast_primary_intliteral(&mut self, father_node: &mut Node) {
        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "INTLITERAL" {
            father_node.add_child(Node::new("INTLITERAL".to_string(), token_value));
            self.tokens.consume_token();
        }
    }

    // <primary> ::= ID
    fn _ast_primary_id(&mut self, father_node: &mut Node) {
        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "ID" {
            father_node.add_child(Node::new("ID".to_string(), token_value));
            self.tokens.consume_token();
        }
    }

    // <primary> ::= LPAREN <expression> RPAREN
    fn _ast_primary_paren(&mut self, father_node: &mut Node) {
        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "LPAREN" {
            self.tokens.consume_token();
        }

        self._ast_expression(father_node);

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "RPAREN" {
            self.tokens.consume_token();
        }
    }

    /*
    CST
     */

    pub fn generate_concrete_syntax_tree(&mut self) {
        let mut root_node = Node::new(
            "ConcreteSyntaxTree".to_string(),
            "ConcreteSyntaxTree".to_string(),
        );
        self.tokens.reset();
        crate::node4m::reset_global_counter();

        self._start(&mut root_node);
        self.concrete_syntax_tree = root_node;
    }

    // <start> ::= <program> SCANEOF
    fn _start(&mut self, father_node: &mut Node) {
        let mut start_node = Node::new("<start>".to_string(), "START".to_string());

        self._program(&mut start_node);

        if let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "SCANEOF" {
                self.tokens.consume_token();
                start_node.add_child(Node::new("SCANEOF".to_string(), "SCANEOF".to_string()));
            }
        }

        father_node.add_child(start_node);
    }

    // <program> ::= BEGIN <statement_list> END
    fn _program(&mut self, father_node: &mut Node) {
        let mut program_node = Node::new("<program>".to_string(), "PROGRAM".to_string());

        if let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "BEGIN" {
                self.tokens.consume_token();
                program_node.add_child(Node::new("BEGIN".to_string(), "BEGIN".to_string()));
            }
        }

        self._statement_list(&mut program_node);

        if let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "END" {
                self.tokens.consume_token();
                program_node.add_child(Node::new("END".to_string(), "END".to_string()));
            }
        }

        father_node.add_child(program_node);
    }

    // <statement_list> ::= <statement> { <statement> }
    fn _statement_list(&mut self, father_node: &mut Node) {
        let mut statement_list_node =
            Node::new("<statement list>".to_string(), "STATEMENT_LIST".to_string());

        self._statement(&mut statement_list_node);
        while let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "ID"
                || token_type.as_str() == "READ"
                || token_type.as_str() == "WRITE"
            {
                self._statement(&mut statement_list_node);
            } else {
                break;
            }
        }

        father_node.add_child(statement_list_node);
    }

    // <statement> ::= ID ASSIGNOP <expression> SEMICOLON
    //              | READ LPAREN <id_list> RPAREN SEMICOLON
    //              | WRITE LPAREN <expression_list> RPAREN SEMICOLON
    fn _statement(&mut self, father_node: &mut Node) {
        let mut statement_node = Node::new("<statement>".to_string(), "STATEMENT".to_string());

        let (token_type, _) = self.tokens.next_token().unwrap();

        match token_type.as_str() {
            "ID" => self._statement_id(&mut statement_node),
            "READ" => self._statment_read(&mut statement_node),
            "WRITE" => self._statment_write(&mut statement_node),
            _ => (),
        }

        father_node.add_child(statement_node);
    }

    // <statment> ::= ID ASSIGNOP <expression> SEMICOLON
    fn _statement_id(&mut self, father_node: &mut Node) {
        let (_, token_value) = self.tokens.next_token().unwrap();
        father_node.add_child(Node::new("ID".to_string(), token_value));
        self.tokens.consume_token();

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "ASSIGNOP" {
            self.tokens.consume_token();
            father_node.add_child(Node::new("ASSIGNOP".to_string(), ":=".to_string()));
        }

        self._expression(father_node);

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "SEMICOLON" {
            self.tokens.consume_token();
            father_node.add_child(Node::new("SEMICOLON".to_string(), ";".to_string()));
        }
    }

    // <statment> ::= READ LPAREN <id_list> RPAREN SEMICOLON
    fn _statment_read(&mut self, father_node: &mut Node) {
        father_node.add_child(Node::new("READ".to_string(), "READ".to_string()));
        self.tokens.consume_token();

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "LPAREN" {
            father_node.add_child(Node::new("LPAREN".to_string(), "(".to_string()));
            self.tokens.consume_token();
        }

        self._id_list(father_node);

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "RPAREN" {
            father_node.add_child(Node::new("RPAREN".to_string(), ")".to_string()));
            self.tokens.consume_token();
        }

        let (token_type, _) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "SEMICOLON" {
            father_node.add_child(Node::new("SEMICOLON".to_string(), ";".to_string()));
            self.tokens.consume_token();
        }
    }

    // <statment> ::= WRITE LPAREN <expression_list> RPAREN SEMICOLON
    fn _statment_write(&mut self, father_node: &mut Node) {
        father_node.add_child(Node::new("WRITE".to_string(), "WRITE".to_string()));
        self.tokens.consume_token();

        father_node.add_child(Node::new("LPAREN".to_string(), "(".to_string()));
        self.tokens.consume_token();

        self._expression_list(father_node);

        father_node.add_child(Node::new("RPAREN".to_string(), ")".to_string()));
        self.tokens.consume_token();

        father_node.add_child(Node::new("SEMICOLON".to_string(), ";".to_string()));
        self.tokens.consume_token();
    }

    // <id_list> ::= ID { COMMA ID }
    fn _id_list(&mut self, father_node: &mut Node) {
        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "ID" {
            father_node.add_child(Node::new("ID".to_string(), token_value));
            self.tokens.consume_token();
        }

        while let Some(token) = self.tokens.next_token() {
            let (token_type, token_value) = token;
            if token_type.as_str() == "COMMA" {
                father_node.add_child(Node::new("COMMA".to_string(), token_value));
                self.tokens.consume_token();
                let (token_type, token_value) = self.tokens.next_token().unwrap();
                if token_type.as_str() == "ID" {
                    father_node.add_child(Node::new("ID".to_string(), token_value));
                    self.tokens.consume_token();
                }
            } else {
                break;
            }
        }
    }

    // <expression_list> ::= <expression> { COMMA <expression> }
    fn _expression_list(&mut self, father_node: &mut Node) {
        let mut expression_list_node = Node::new(
            "<expression list>".to_string(),
            "EXPRESSION_LIST".to_string(),
        );

        self._expression(&mut expression_list_node);

        while let Some(token) = self.tokens.next_token() {
            let (token_type, token_value) = token;
            if token_type.as_str() == "COMMA" {
                expression_list_node.add_child(Node::new("COMMA".to_string(), token_value));
                self.tokens.consume_token();
                self._expression(&mut expression_list_node);
            } else {
                break;
            }
        }

        father_node.add_child(expression_list_node);
    }

    // <expression> ::= <primary> { <add_op> <primary> }
    fn _expression(&mut self, father_node: &mut Node) {
        let mut expression_node = Node::new("<expression>".to_string(), "EXPRESSION".to_string());

        self._primary(&mut expression_node);

        while let Some(token) = self.tokens.next_token() {
            let (token_type, _) = token;
            if token_type.as_str() == "PLUSOP" || token_type.as_str() == "MINUSOP" {
                self._add_op(&mut expression_node);
                self._primary(&mut expression_node);
            } else {
                break;
            }
        }

        father_node.add_child(expression_node);
    }

    // <primary> ::= INTLITERAL | ID | LPAREN <expression> RPAREN
    fn _primary(&mut self, father_node: &mut Node) {
        let mut primary_node = Node::new("<primary>".to_string(), "PRIMARY".to_string());

        let (token_type, _) = self.tokens.next_token().unwrap();

        match token_type.as_str() {
            "INTLITERAL" => self._primary_intliteral(&mut primary_node),
            "ID" => self._primary_id(&mut primary_node),
            "LPAREN" => self._primary_paren(&mut primary_node),
            _ => (),
        }

        father_node.add_child(primary_node);
    }

    // <primary> ::= INTLITERAL
    fn _primary_intliteral(&mut self, father_node: &mut Node) {
        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "INTLITERAL" {
            father_node.add_child(Node::new("INTLITERAL".to_string(), token_value));
            self.tokens.consume_token();
        }
    }

    // <primary> ::= ID
    fn _primary_id(&mut self, father_node: &mut Node) {
        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "ID" {
            father_node.add_child(Node::new("ID".to_string(), token_value));
            self.tokens.consume_token();
        }
    }

    // <primary> ::= LPAREN <expression> RPAREN
    fn _primary_paren(&mut self, father_node: &mut Node) {
        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "LPAREN" {
            father_node.add_child(Node::new("LPAREN".to_string(), token_value));
            self.tokens.consume_token();
        }

        self._expression(father_node);

        let (token_type, token_value) = self.tokens.next_token().unwrap();
        if token_type.as_str() == "RPAREN" {
            father_node.add_child(Node::new("RPAREN".to_string(), token_value));
            self.tokens.consume_token();
        }
    }

    // <add_op> ::= PLUSOP | MINUSOP
    fn _add_op(&mut self, father_node: &mut Node) {
        let mut add_op_node = Node::new("<addop>".to_string(), "ADDOP".to_string());

        if let Some(token) = self.tokens.next_token() {
            let (token_type, token_value) = token;
            if token_type.as_str() == "PLUSOP" || token_type.as_str() == "MINUSOP" {
                add_op_node.add_child(Node::new(token_type, token_value));
                self.tokens.consume_token();
            }
        }

        father_node.add_child(add_op_node);
    }
}
