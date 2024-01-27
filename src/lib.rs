#[derive(Debug, PartialEq)]
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

impl Token {
    pub fn is_ident(&self) -> bool {
        match *self {
            Self::Ident(_) | Self::Int(_) | Self::Function | Self::Let => true,
            _ => false,
        }
    }
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

    pub fn read_ident(&mut self) -> String {
        let pos = self.position;
        while let Some('a'..='z' | 'A'..='Z' | '_') = self.ch {
            self.read_char();
        }
        return String::from(&self.input[pos..self.position]);
    }

    pub fn read_int(&mut self) -> String {
        let pos = self.position;
        while let Some('0'..='9') = self.ch {
            self.read_char();
        }
        return String::from(&self.input[pos..self.position]);
    }

    pub fn lookup_ident(ident: String) -> Token {
        match ident.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            _ => Token::Ident(ident),
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_some_and(|ch| ch.is_whitespace()) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = self.ch.map_or(Token::Eof, |ch| match ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            'a'..='z' | 'A'..='Z' | '_' => Lexer::lookup_ident(self.read_ident()),
            '0'..='9' => Token::Int(self.read_int()),
            _ => Token::Illegal,
        });

        if !token.is_ident() {
            self.read_char();
        }

        return token;
    }
}
