use std::collections::{hash_map::Values, HashMap};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

struct Keywords<'a> {
    keywords: HashMap<&'a str, Token>,
}

impl<'a> Keywords<'a> {
    fn new() -> Self {
        Self {
            keywords: HashMap::from([
                ("fn", Token::Function),
                ("let", Token::Let),
                ("true", Token::True),
                ("false", Token::False),
                ("if", Token::If),
                ("else", Token::Else),
                ("return", Token::Return),
            ]),
        }
    }

    fn get(&self, key: &str) -> Option<Token> {
        return self.keywords.get(key).cloned();
    }

    fn values(&self) -> Values<'_, &str, Token> {
        return self.keywords.values();
    }
}

impl Token {
    pub fn is_keyword(&self) -> bool {
        return Keywords::new().values().any(|token| self == token);
    }

    pub fn is_ident(&self) -> bool {
        match *self {
            Self::Ident(_) | Self::Int(_) => true,
            _ if self.is_keyword() => true,
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

    pub fn peek(&self) -> Option<char> {
        return self.input.chars().nth(self.read_position);
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
        Keywords::new().get(&ident).unwrap_or(Token::Ident(ident))
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_some_and(|ch| ch.is_whitespace()) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = self.ch.map_or(Token::Eof, |ch| match ch {
            '=' => {
                if let Some('=') = self.peek() {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if let Some('=') = self.peek() {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '>' => {
                if let Some('=') = self.peek() {
                    self.read_char();
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            }
            '<' => {
                if let Some('=') = self.peek() {
                    self.read_char();
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            }
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            'a'..='z' | 'A'..='Z' | '_' => Self::lookup_ident(self.read_ident()),
            '0'..='9' => Token::Int(self.read_int()),
            _ => Token::Illegal,
        });

        if !token.is_ident() {
            self.read_char();
        }

        return token;
    }
}
