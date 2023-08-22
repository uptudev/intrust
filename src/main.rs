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
}
