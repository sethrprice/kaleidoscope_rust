use inkwell::context::Context;

fn main() {
    let context = Context::create();
    let module = context.create_module("kaleidoscope");
    let builder = context.create_builder();

    println!("LLVM Module Created: {:?}", module);
}

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

        // recognise identifiers
        if self.current_char.map_or(false, |c| c.is_alphabetic()) {}
    }
}
