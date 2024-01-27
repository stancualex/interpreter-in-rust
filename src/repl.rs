use anyhow::Result;
use std::io::{BufRead, Write};

use crate::lexer::{Lexer, Token};

const MESSAGE: &'static str = "\
    Hello user! This is the monkey programming language.\n\
    Feel free to type in commands!\n\
";
const PROMPT: &'static str = ">> ";

pub fn start(mut reader: impl BufRead, mut writer: impl Write) -> Result<()> {
    write!(writer, "{}", MESSAGE)?;
    loop {
        write!(writer, "{}", PROMPT)?;
        writer.flush()?;

        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut lexer = Lexer::new(line);
        let mut token = lexer.next_token();
        while token != Token::Eof {
            writeln!(writer, "{:?}", token)?;
            token = lexer.next_token();
        }
    }
}
