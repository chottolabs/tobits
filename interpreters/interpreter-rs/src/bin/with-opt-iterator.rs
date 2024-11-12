use lexer::with_opt_iterator::{Tag, Tokenizer};
use std::env;
use std::io::{self};

fn run_interpreter() -> io::Result<()> {
    println!("(zig-like) zlox interpreter v0.0.1");
    println!("Type your code below. Press Ctrl+D (Unix) or Ctrl+Z (Windows) to end input.");

    let args: Vec<String> = env::args().collect();
    let buffer = std::fs::read(&args[1])?;

    for token in Tokenizer::new(&buffer) {
        match token.tag {
            Tag::Invalid => {
                println!("Invalid token at position {}.", token.loc.start);
                break;
            }
            _ => {
                // // Optionally, extract the actual text
                // let lexeme =
                //     std::str::from_utf8(&buffer[token.loc.start..token.loc.end]).unwrap_or("");
                // println!("{:?} '{}'", token.tag, lexeme);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run_interpreter() {
        eprintln!("Error: {}", e);
    }
}
