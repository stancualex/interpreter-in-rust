#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
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

    pub fn walk_back(&mut self) {
        self.read_position -= 1;
        self.position = self.read_position - 1;
        self.ch = self.input.chars().nth(self.read_position);
    }

    pub fn peek(&self) -> Option<char> {
        return self.input.chars().nth(self.read_position);
    }

    pub fn read_ident(&mut self) -> String {
        let pos = self.position;
        while let Some('a'..='z' | 'A'..='Z' | '_') = self.ch {
            self.read_char();
        }
        self.walk_back();
        return String::from(&self.input[pos..self.read_position]);
    }

    pub fn read_int(&mut self) -> String {
        let pos = self.position;
        while let Some('0'..='9') = self.ch {
            self.read_char();
        }
        self.walk_back();
        return String::from(&self.input[pos..self.read_position]);
    }

    pub fn lookup_ident(ident: String) -> Token {
        match ident.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
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

        self.read_char();

        return token;
    }
}
