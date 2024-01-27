#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            ..Default::default()
        };
        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = self.ch.map_or(Token::Eof, |ch| match ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            _ => Token::Illegal,
        });
        self.read_char();
        return token;
    }
}
