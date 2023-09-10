pub mod parser {
    use crate::libs::{ast::ast::Statement, lexer::lexer::Token};

    use super::super::{ast::ast, lexer::lexer};

    pub struct Parser {
        lexer: lexer::Lexer,
        curr_token: lexer::Token,
        peek_token: lexer::Token,
        pub(crate) errors: Vec<String>,
    }

    impl Parser {
        pub fn new(lexer: lexer::Lexer) -> Self {
            let mut p = Parser {
                lexer,
                // garbage tokens for init, pushed out immediately
                curr_token: lexer::Token::ILLEGAL,
                peek_token: lexer::Token::ILLEGAL,
                errors: Vec::new(),
            };
            // set both tokens to the first 2.
            p.next_token();
            p.next_token();
            p
        }

        fn next_token(&mut self) {
            self.curr_token = self.peek_token.clone();
            self.peek_token = self.lexer.next_token();
        }

        pub fn parse_program(&mut self) -> Option<ast::Program> {
            let mut program = ast::Program{
                statements: Vec::new(),
            };

            while self.curr_token != lexer::Token::EOF {
                let statement = self.parse_statement();
                if let Some(statement) = statement {
                    program.statements.push(statement);
                }
                self.next_token();
            }

            Some(program)
        }

        fn parse_statement(&mut self) -> Option<Statement> {
            match self.curr_token {
                lexer::Token::LET => {
                    let statement = match self.parse_let_statement() {
                        Some(let_statement) => {Statement::Let(Box::new(let_statement))},
                        None => {return None},
                    };

                    Some(statement)
                },
                lexer::Token::RETURN => {
                    let statement = match self.parse_return_statement() {
                        Some(return_statement) => {Statement::Return(Box::new(return_statement))},
                        None => {return None}
                    };

                    Some(statement)
                },
                _ => {None},
            }
        }

        fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
            if !self.expect_peek_ident() {
                self.errors.push(format!("missing identifier"));
                return None;
            }

            if self.peek_token != lexer::Token::ASSIGN {
                self.errors.push(format!("missing assignment"));
                return None;
            }

            let statement = ast::LetStatement {
                token: self.curr_token.clone(),
                name: Box::new(ast::Identifier{
                    token: self.curr_token.clone(), 
                    value: self.curr_token.literal(),
                }),
                value: None,
            };

            while self.curr_token != lexer::Token::SEMI {
                if self.peek_token == lexer::Token::EOF {
                    self.errors.push(format!("missing semicolon"));
                    return None;
                }
                self.next_token();
            }

            Some(statement)
        }

        fn parse_return_statement(&mut self) -> Option<ast::ReturnStatement> {
            let statement = ast::ReturnStatement {
                token: self.curr_token.clone(),
                return_val: None,
            };

            while self.curr_token != Token::SEMI && self.curr_token != Token::EOF {
                self.next_token();
            }

            return Some(statement);
        }

        fn expect_peek_ident(&mut self) -> bool {
            match self.peek_token {
                lexer::Token::IDENT(_) => {
                    self.next_token();
                    return true;
                },
                _ => {false},
            }
        }
    }
}
