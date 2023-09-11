mod libs;

fn main() {
    repl::start();
}

mod repl {
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
                }
            }
        }
    }
}
