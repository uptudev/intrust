pub mod ast {
    use super::super::lexer::lexer::Token;

    pub enum Statement {
        Let(Box<LetStatement>),
        Invalid,
    }

    impl Node for Statement {
        fn token_literal(&self) -> String {
            match self {
                Statement::Let(statement) => {statement.token_literal()},
                _ => {"".to_string()},
            }
        }
    }

    pub trait Node {
        fn token_literal(&self) -> String;
    }

    pub trait Expression: Node {
        fn expression_node(&self);
    }

    pub struct Program {
        pub statements: Vec<Statement>,
    }

    impl Node for Program {
        fn token_literal(&self) -> String {
            if !self.statements.is_empty() {
                return self.statements[0].token_literal();
            } else {
                return String::new();
            }
        }
    }

    pub struct LetStatement {
        pub token: Token,
        pub name: Box<Identifier>,
        pub value: Option<Box<dyn Expression>>,
    }

    impl LetStatement {

    }

    impl Node for LetStatement {
        fn token_literal(&self) -> String {
            return self.token.literal();
        }
    }

    pub struct Identifier {
        pub token: Token,
        pub value: String,
    }

    impl Expression for Identifier {
        fn expression_node(&self) {
            //idk why this is empty rn
        }
    }

    impl Node for Identifier {
        fn token_literal(&self) -> String {
            return self.token.literal();
        }
    }
}
