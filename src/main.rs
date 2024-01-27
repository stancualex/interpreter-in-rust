use interpreter::Lexer;

fn main() {
    let mut lexer = Lexer::new(String::from("Hello"));
    lexer.read_char();
    lexer.read_char();
    lexer.read_char();
    lexer.read_char();
    lexer.read_char();
    println!("{:?}", lexer);
}
