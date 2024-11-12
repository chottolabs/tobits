use lexer::with_opt_unstable::{Tag, Tokenizer};
use std::env;
use std::io::{self};

fn run_interpreter() -> io::Result<()> {
    println!("(zig-like) zlox interpreter v0.0.1");
    println!("Type your code below. Press Ctrl+D (Unix) or Ctrl+Z (Windows) to end input.");

    let args: Vec<String> = env::args().collect();
    let buffer = std::fs::read(&args[1])?;
    let mut tokenizer = Tokenizer::new(&buffer);

    loop {
        match tokenizer.next_token() {
            Some(token) => match token.tag {
                Tag::Eof => break,
                Tag::Invalid => {
                    println!("Invalid token at position {}.", token.loc.start);
                    break;
                }
                _ => {
                    // Optionally, extract the actual text
                    // let lexeme =
                    //     std::str::from_utf8(&buffer[token.loc.start..token.loc.end]).unwrap_or("");
                    // println!("{:?} '{}'", token.tag, lexeme);
                }
            },
            None => break,
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_interpreter() {
        eprintln!("Error: {}", e);
    }
}
