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
