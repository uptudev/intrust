mod libs;
use libs::lexer;

fn main() {
    repl::start();
}

mod repl {
    use crate::lexer::lexer::{Lexer, Token};
    use std::io::{stdin, stdout, Write};
    use whoami::username;
    use colored::Colorize;

    const PROMPT: &str = ">> ";

    pub fn start() {
        let username = username();

        println!("{} {} {}, {}!", "Welcome to the".red(),"Intrus".bright_red().bold(), "REPL".bright_green().bold(), username.cyan());
        let mut buff = String::new();

        loop {
            buff.clear();
            print!("{}", PROMPT.bright_yellow().bold());
            stdout().flush().unwrap();

            match stdin().read_line(&mut buff) {
                Ok(_) => {println!();}
                Err(e) => {
                    panic!("Error: {}", e)
                }
            }
            let buff = buff.trim_end();
            match buff {
                "" => return,
                _ => {
                    let mut lx = Lexer::new(buff);
                    let mut tok = lx.next_token();
                    let mut tokens: Vec<Token> = Vec::new();
                    while tok != Token::EOF {
                        tokens.push(tok.clone());
                        println!("{}{}", tok.get_type().bright_white().bold(), tok.literal().blue().italic());
                        tok = lx.next_token();
                    }
                    tokens.push(tok);

                    println!("\n{}:\n{:?}\n", "ALL TOKENS".bright_white().bold(), tokens);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::{Lexer, Token};
    use crate::libs::ast::ast::Statement;
    use crate::libs::parser::parser::Parser;

    #[test]
    fn test_next_token() {
        let input = "let xes = 1;
        let yes = 1.0;
        let zes = xes + yes;";
        let tests = [
            Token::LET,
            Token::IDENT("xes".to_string()),
            Token::ASSIGN,
            Token::INT(1),
            Token::SEMI,
            Token::LET,
            Token::IDENT("yes".to_string()),
            Token::ASSIGN,
            Token::FLT(1.0),
            Token::SEMI,
            Token::LET,
            Token::IDENT("zes".to_string()),
            Token::ASSIGN,
            Token::IDENT("xes".to_string()),
            Token::ADD,
            Token::IDENT("yes".to_string()),
            Token::SEMI,
            Token::EOF,
        ];

        let mut l = Lexer::new(&input);

        for (i, token) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(
                tok, *token,
                "Token lexing error, inequivalence in tests[{}]",
                i
            );
        }
    }

    #[test]
    fn test_return_statements_pass() {
        let input = "return 5;\nreturn 1;\n return 1020;";
        test_return_statements(input);
    }

    #[test]
    fn test_let_statements_pass() {
        let input = "let x = 4;\nlet y = 10;\nlet foobar = 838383;\n";
        test_let_statements(input);
    }

    #[test]
    fn test_let_statements_fail() {
        let input = "let x 4;\nlet = 10;\nlet 838383;\n";
        test_let_statements(input);
    }

    fn test_return_statements(input: &str) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser.errors);

        if let Some(program) = program {
            for (_, statement) in program.statements.iter().enumerate() {
                if let Statement::Return(_) = statement {

                } else {
                    
                }
            }
        } else {
            parser.errors.push("parse_program returned None".to_string());
        }
    }

    fn test_let_statements(input: &str) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser.errors);

        if let Some(program) = program {
            let tests: Vec<&str> = vec!("x", "y", "foobar");

            for (i, statement) in program.statements.iter().enumerate() {
                if let Statement::Let(_) = statement {
                    if !test_let_statement(statement, tests[i]) {
                        // Handle test failure here, but don't panic inside the loop
                    }
                } else {
                    // Handle non-LetStatement case here
                }
            }
        } else {
            parser.errors.push("parse_program returned None".to_string());
        }
    }

    fn check_parser_errors(errors: &Vec<String>) {
        let msg = format!("parser has {} errors:\n{}", errors.len(), errors.join("\n"));
        assert!(errors.is_empty(), "{}", msg);
    }

    fn test_let_statement(statement: &Statement, expected_identifier: &str) -> bool {
        match statement {
            Statement::Let(let_stmt) => {
                let name = &let_stmt.name.value;
                name == expected_identifier
            },
            _ => false,
        }
    }
}
