use anyhow::Result;
use interpreter::{Lexer, Token};

#[test]
fn test_next_token() -> Result<()> {
    let input = "=+(){},;".to_string();
    let mut lexer = Lexer::new(input);

    let tokens = [
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Semicolon,
        Token::Eof,
    ];

    for token in tokens {
        let next = lexer.next_token();
        println!("expected {:?}, received {:?}", token, next);
        assert_eq!(token, next);
    }

    Ok(())
}

#[test]
fn test_next_complete() -> Result<()> {
    let input = r#"let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        10 >= 9;
        9 <= 10;"#
        .to_string();
    let mut lexer = Lexer::new(input);

    let tokens = [
        Token::Let,
        Token::Ident("five".into()),
        Token::Assign,
        Token::Int("5".into()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".into()),
        Token::Assign,
        Token::Int("10".into()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".into()),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident("x".into()),
        Token::Comma,
        Token::Ident("y".into()),
        Token::RParen,
        Token::LBrace,
        Token::Ident("x".into()),
        Token::Plus,
        Token::Ident("y".into()),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".into()),
        Token::Assign,
        Token::Ident("add".into()),
        Token::LParen,
        Token::Ident("five".into()),
        Token::Comma,
        Token::Ident("ten".into()),
        Token::RParen,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int("5".into()),
        Token::Semicolon,
        Token::Int("5".into()),
        Token::LessThan,
        Token::Int("10".into()),
        Token::GreaterThan,
        Token::Int("5".into()),
        Token::Semicolon,
        Token::If,
        Token::LParen,
        Token::Int("5".into()),
        Token::LessThan,
        Token::Int("10".into()),
        Token::RParen,
        Token::LBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RBrace,
        Token::Int("10".into()),
        Token::Equal,
        Token::Int("10".into()),
        Token::Semicolon,
        Token::Int("10".into()),
        Token::NotEqual,
        Token::Int("9".into()),
        Token::Semicolon,
        Token::Int("10".into()),
        Token::GreaterEqual,
        Token::Int("9".into()),
        Token::Semicolon,
        Token::Int("9".into()),
        Token::LessEqual,
        Token::Int("10".into()),
        Token::Semicolon,
        Token::Eof,
    ];

    for token in tokens {
        let next = lexer.next_token();
        println!("expected {:?}, received {:?}", token, next);
        assert_eq!(token, next);
    }

    Ok(())
}
