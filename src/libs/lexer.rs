pub mod lexer {
    use core::fmt;

    pub struct Lexer {
        input: String,
        position: usize,
        next_position: usize,
        ch: Option<char>,
    }

    impl Lexer {
        pub fn new(input: &str) -> Self {
            let mut lexer = Lexer {
                input: input.to_string(),
                position: 0,
                next_position: 0,
                ch: None,
            };
            lexer.read_char();
            lexer
        }

        pub fn next_token(&mut self) -> Token {
            self.skip_whitespace();

            let token = match self.ch {
                Some('=') => match self.peek_char() {
                    Some('=') => {self.read_char(); Token::EQUAL},
                    _ => Token::ASSIGN,
                },
                Some('+') => Token::ADD,
                Some('-') => Token::SUBTRACT,
                Some('*') => Token::MULTIPLY,
                Some('/') => Token::DIVIDE,
                Some('%') => Token::MODULO,
                Some('^') => Token::RAISE,
                Some('<') => Token::LTHAN,
                Some('>') => Token::GTHAN,
                Some('!') => match self.peek_char() {
                    Some('=') => {self.read_char(); Token::UNEQUAL},
                    _ => Token::NOT,
                },

                Some('(') => Token::LPAREN,
                Some(')') => Token::RPAREN,
                Some('{') => Token::LBRACE,
                Some('}') => Token::RBRACE,
                Some('[') => Token::LBRACK,
                Some(']') => Token::RBRACK,

                Some(',') => Token::COMMA,
                Some(';') => Token::SEMI,

                Some(_) => {
                    let curr_ch = self.ch.unwrap();
                    if is_number(curr_ch){
                        check_for_numbers(&self.read_next_alphanumeric())
                    } else if is_letter(curr_ch) {
                        check_for_keywords(&self.read_next_alphanumeric())
                    } else {
                        Token::ILLEGAL
                    }
                }
                None => Token::EOF,
            };

            self.read_char();
            token
        }

        fn skip_whitespace(&mut self) {
            while let Some(ch) = self.ch {
                match ch {
                    ' ' | '\t' | '\n' | '\r' => self.read_char(),
                    _ => break,
                }
            }
        }

        fn read_char(&mut self) {
            if self.next_position >= self.input.len() {
                self.ch = None;
            } else {
                self.ch = self.input.chars().nth(self.next_position);
            }
            self.position = self.next_position;
            self.next_position += 1;
        }

        fn peek_char(&mut self) -> Option<char> {
            if self.next_position >= self.input.len() {
                None
            } else {
                self.input.chars().nth(self.next_position)
            }
        }

        fn read_next_alphanumeric(&mut self) -> String {
            let position = self.position;
            while let Some(c) = self.ch {
                if is_letter(c) {
                    self.read_char()
                } else {break;}
            }
            self.position -= 1;
            self.next_position -= 1;
            return self.input[position..=self.position].to_string();
        }
    }

    fn is_number(ch: char) -> bool {
        ch.is_digit(10) || ch == '.'
    }
    fn is_letter(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '.'
    }

    fn check_for_keywords(input: &str) -> Token {
        match input {
            "fn"                => Token::FUNCTION,
            "let"               => Token::LET,
            "while"             => Token::WHILE,
            "true"              => Token::BOOL(true),
            "false"             => Token::BOOL(false),
            "if"                => Token::IF,
            "return"            => Token::RETURN,
            "f32"|"single"      => Token::TYPE(Type::F32),
            "f64"|"double"      => Token::TYPE(Type::F64),
            "i8"                => Token::TYPE(Type::I8),
            "i16"               => Token::TYPE(Type::I16),
            "i32"               => Token::TYPE(Type::I32),
            "i64"               => Token::TYPE(Type::I64),
            "i128"              => Token::TYPE(Type::I128),
            "u8"|"byte"         => Token::TYPE(Type::U8),
            "u16"               => Token::TYPE(Type::U16),
            "u32"               => Token::TYPE(Type::U32),
            "u64"               => Token::TYPE(Type::U64),
            "u128"              => Token::TYPE(Type::U128),
            "bool"|"u1"|"bit"   => Token::TYPE(Type::BOOL),
            "char"              => Token::TYPE(Type::CHAR),
            _                   => {

                Token::IDENT(input.to_string())
            },
        }
    }

    fn check_for_numbers(input: &str) -> Token {
        if input.contains('.') {
            Token::FLT(input.parse::<f64>().expect("Error parsing input to float."))
        } else if input.chars().all(char::is_numeric) {
            Token::INT(
                input
                    .parse::<u64>()
                    .expect("Error parsing input to integer.")
            )
        } else if input.chars().all(char::is_alphanumeric) {
            Token::IDENT(input.to_string())
        } else {
            Token::ILLEGAL
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Type {
        F32,
        F64,
        I8,
        I16,
        I32,
        I64,
        I128,
        U8,
        U16,
        U32,
        U64,
        U128,
        BOOL,
        CHAR,
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let inner = match self {
                Type::F32 => "F32",
                Type::F64 => "F64",
                Type::I8  => "I8",
                Type::I16 => "I16",
                Type::I32 => "I32",
                Type::I64 => "I64",
                Type::I128=> "I128",
                Type::U8  => "U8",
                Type::U16 => "U16",
                Type::U32 => "U32",
                Type::U64 => "U64",
                Type::U128=> "U128",
                Type::BOOL=> "BOOL",
                Type::CHAR=> "CHAR",
            };
            write!(f, "{}", inner)
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
        ILLEGAL,
        EOF,

        // Identifiers
        IDENT(String),

        // Primitives
        TYPE(Type),

        // Literals
        BOOL(bool), // bit
        FLT(f64),   // float
        INT(u64),   // uint

        // Operators
        ASSIGN,   // =
        ADD,      // +
        SUBTRACT, // -
        MULTIPLY, // *
        DIVIDE,   // /
        MODULO,   // %
        RAISE,    // ^
        NOT,      // !
        LTHAN,    // <
        GTHAN,    // >
        EQUAL,    // ==
        UNEQUAL,  // !=

        // Delimiters
        COMMA,  // ,
        SEMI,   // ;

        // Blocks
        LPAREN, // (
        RPAREN, // )
        LBRACE, // {
        RBRACE, // }
        LBRACK, // [
        RBRACK, // ]

        // Keywords
        FUNCTION, // fn
        LET,      // let
        WHILE,    // while
        IF,       // if
        RETURN,   // return
    }

    impl Token {
        /// Gets the literal representation of the token, returning the inner value of primitives.
        pub fn literal(&self) -> String {
            match self {
                Token::ILLEGAL  => "ILLEGAL".to_string(),
                Token::EOF      => "EOF".to_string(),

                // Return string representations of inner literal values
                Token::IDENT(i) => i.to_string(),
                Token::TYPE(i)  => {
                    match i {
                        Type::F32 => "f32".to_string(),
                        Type::F64 => "f64".to_string(),
                        Type::I8  => "i8".to_string(),
                        Type::I16 => "i16".to_string(),
                        Type::I32 => "i32".to_string(),
                        Type::I64 => "i64".to_string(),
                        Type::I128=> "i128".to_string(),
                        Type::U8  => "u8".to_string(),
                        Type::U16 => "u16".to_string(),
                        Type::U32 => "u32".to_string(),
                        Type::U64 => "u64".to_string(),
                        Type::U128=> "u128".to_string(),
                        Type::BOOL=> "bool".to_string(),
                        Type::CHAR=> "char".to_string(),

                    }
                },
                Token::BOOL(i)  => i.to_string(),
                Token::INT(i)   => i.to_string(),
                Token::FLT(i)   => i.to_string(),

                Token::ASSIGN   => "=".to_string(),
                Token::ADD      => "+".to_string(),
                Token::SUBTRACT => "-".to_string(),
                Token::MULTIPLY => "*".to_string(),
                Token::DIVIDE   => "/".to_string(),
                Token::MODULO   => "%".to_string(),
                Token::RAISE    => "^".to_string(),
                Token::NOT      => "!".to_string(),
                Token::LTHAN    => "<".to_string(),
                Token::GTHAN    => ">".to_string(),
                Token::EQUAL    => "==".to_string(),
                Token::UNEQUAL  => "!=".to_string(),

                Token::COMMA    => ",".to_string(),
                Token::SEMI     => ";".to_string(),
                Token::LPAREN   => "(".to_string(),
                Token::RPAREN   => ")".to_string(),
                Token::LBRACE   => "{".to_string(),
                Token::RBRACE   => "}".to_string(),
                Token::LBRACK   => "[".to_string(),
                Token::RBRACK   => "]".to_string(),

                Token::FUNCTION => "fn".to_string(),
                Token::LET      => "let".to_string(),
                Token::WHILE    => "while".to_string(),
                Token::IF       => "if".to_string(),
                Token::RETURN   => "return".to_string(),
            }
        }

        pub fn get_type(&self) -> String {
            match self {
                Token::ILLEGAL  => "ILLEGAL\t\t".to_string(),
                Token::EOF      => "EOF\t\t".to_string(),

                // Return string representations of inner literal values
                Token::IDENT(_) => "IDENT\t\t".to_string(),
                Token::TYPE(i)  => i.to_string(),
                Token::BOOL(_)  => "BOOLEAN\t\t".to_string(),
                Token::INT(_)   => "INTEGER\t\t".to_string(),
                Token::FLT(_)   => "FLOAT\t\t".to_string(),

                Token::ASSIGN   => "ASSIGN\t\t".to_string(),
                Token::ADD      => "ADD\t\t".to_string(),
                Token::SUBTRACT => "SUBTRACT\t".to_string(),
                Token::MULTIPLY => "MULTIPLY\t".to_string(),
                Token::DIVIDE   => "DIVIDE\t\t".to_string(),
                Token::MODULO   => "MODULO\t\t".to_string(),
                Token::RAISE    => "RAISE\t\t".to_string(),
                Token::NOT      => "NOT\t\t".to_string(),
                Token::LTHAN    => "LTHAN\t\t".to_string(),
                Token::GTHAN    => "GTHAN\t\t".to_string(),
                Token::EQUAL    => "EQUAL\t\t".to_string(),
                Token::UNEQUAL  => "UNEQUAL\t\t".to_string(),

                Token::COMMA    => "COMMA\t\t".to_string(),
                Token::SEMI     => "SEMI\t\t".to_string(),
                Token::LPAREN   => "LPAREN\t\t".to_string(),
                Token::RPAREN   => "RPAREN\t\t".to_string(),
                Token::LBRACE   => "LBRACE\t\t".to_string(),
                Token::RBRACE   => "RBRACE\t\t".to_string(),
                Token::LBRACK   => "LBRACK\t\t".to_string(),
                Token::RBRACK   => "RBRACK\t\t".to_string(),

                Token::FUNCTION => "FUNCTION\t".to_string(),
                Token::LET      => "LET\t\t".to_string(),
                Token::WHILE    => "WHILE\t\t".to_string(),
                Token::IF       => "IF\t\t".to_string(),
                Token::RETURN   => "RETURN\t\t".to_string(),
            }
        }
    }
}
