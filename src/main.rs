use interpreter::Lexer;

fn main() {
    let mut lexer = Lexer::new(String::from("Hello World"));
    let tok = lexer.next_token();
    println!("{:?}", tok);
}
