use std::io;
use anyhow::Result;

pub mod lexer;
pub mod repl;

fn main() -> Result<()> {
    repl::start(io::stdin().lock(), io::stdout().lock())?;
    Ok(())
}
