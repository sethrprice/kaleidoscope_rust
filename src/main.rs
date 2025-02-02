// use inkwell::context::Context;

fn main() {
    // let context = Context::create();
    // let module = context.create_module("kaleidoscope");
    // let builder = context.create_builder();

    // println!("LLVM Module Created: {:?}", module);

    let source = "def foo 3.14 extern";
    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.get_tok();
        println!("{:?}", token);
        if token == Token::Eof {
            break;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    // End of file
    Eof,

    // Commands
    Def,
    Extern,

    // Primary
    Identifier(String),
    Number(f64),
    Char(char),
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            pos: 0,
            current_char: None,
        };
        lexer.advance();
        return lexer;
    }

    fn advance(&mut self) {
        self.current_char = self.input.chars().nth(self.pos);
        self.pos += 1;
    }

    pub fn get_tok(&mut self) -> Token {
        // skip whitespace
        while self.current_char.map_or(false, |c| c.is_whitespace()) {
            self.advance()
        }

        // recognise identifiers and keywords
        if let Some(c1) = self.current_char {
            if c1.is_alphabetic() {
                let mut identifier = String::new();
                while let Some(c2) = self.current_char {
                    if c2.is_alphabetic() {
                        identifier.push(c2);
                        self.advance();
                    } else {
                        break;
                    }
                }
                return match identifier.as_str() {
                    "def" => Token::Def,
                    "extern" => Token::Extern,
                    _ => Token::Identifier(identifier),
                };
            }

            // recognise numbers
            if c1.is_digit(10) || c1 == '.' {
                let mut num_str = String::new();
                while let Some(c2) = self.current_char {
                    if c2.is_digit(10) || c2 == '.' {
                        num_str.push(c2);
                        self.advance();
                    } else {
                        break;
                    }
                }
                return Token::Number(num_str.parse().unwrap());
            }

            // single character tokens e.g. operators, punctuation
            self.advance();
            return Token::Char(c1);
        }
        return Token::Eof;
    }
}
