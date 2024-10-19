use std::env;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    KeywordAnd,
    KeywordClass,
    KeywordElse,
    KeywordFalse,
    KeywordFun,
    KeywordFor,
    KeywordIf,
    KeywordNil,
    KeywordOr,
    KeywordPrint,
    KeywordReturn,
    KeywordSuper,
    KeywordThis,
    KeywordTrue,
    KeywordVar,
    KeywordWhile,

    Eof,
    Invalid,
}

impl Tag {
    fn from_keyword(keyword: &str) -> Option<Tag> {
        match keyword {
            "and" => Some(Tag::KeywordAnd),
            "class" => Some(Tag::KeywordClass),
            "else" => Some(Tag::KeywordElse),
            "false" => Some(Tag::KeywordFalse),
            "fun" => Some(Tag::KeywordFun),
            "for" => Some(Tag::KeywordFor),
            "if" => Some(Tag::KeywordIf),
            "nil" => Some(Tag::KeywordNil),
            "or" => Some(Tag::KeywordOr),
            "print" => Some(Tag::KeywordPrint),
            "return" => Some(Tag::KeywordReturn),
            "super" => Some(Tag::KeywordSuper),
            "this" => Some(Tag::KeywordThis),
            "true" => Some(Tag::KeywordTrue),
            "var" => Some(Tag::KeywordVar),
            "while" => Some(Tag::KeywordWhile),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tag: Tag,
    pub loc: Loc,
}

pub struct Tokenizer<'a> {
    buffer: &'a [u8],
    index: usize,
    line: usize, // Optional: For better error reporting
}

impl<'a> Tokenizer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Tokenizer {
            buffer,
            index: 0,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.index >= self.buffer.len() {
            return Some(Token {
                tag: Tag::Eof,
                loc: Loc {
                    start: self.index,
                    end: self.index,
                },
            });
        }

        let start = self.index;
        let c = self.current_char();

        let tag = match c {
            b'(' => {
                self.advance();
                Tag::LeftParen
            }
            b')' => {
                self.advance();
                Tag::RightParen
            }
            b'{' => {
                self.advance();
                Tag::LeftBrace
            }
            b'}' => {
                self.advance();
                Tag::RightBrace
            }
            b',' => {
                self.advance();
                Tag::Comma
            }
            b'.' => {
                self.advance();
                Tag::Dot
            }
            b'-' => {
                self.advance();
                Tag::Minus
            }
            b'+' => {
                self.advance();
                Tag::Plus
            }
            b';' => {
                self.advance();
                Tag::Semicolon
            }
            b'*' => {
                self.advance();
                Tag::Star
            }
            b'!' => {
                self.advance();
                if self.match_char(b'=') {
                    Tag::BangEqual
                } else {
                    Tag::Bang
                }
            }
            b'=' => {
                self.advance();
                if self.match_char(b'=') {
                    Tag::EqualEqual
                } else {
                    Tag::Equal
                }
            }
            b'<' => {
                self.advance();
                if self.match_char(b'=') {
                    Tag::LessEqual
                } else {
                    Tag::Less
                }
            }
            b'>' => {
                self.advance();
                if self.match_char(b'=') {
                    Tag::GreaterEqual
                } else {
                    Tag::Greater
                }
            }
            b'0'..=b'9' => {
                return self.number(start);
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.identifier(start);
            }
            _ => {
                self.advance();
                Tag::Invalid
            }
        };

        Some(Token {
            tag,
            loc: Loc {
                start,
                end: self.index,
            },
        })
    }

    fn current_char(&self) -> u8 {
        self.buffer[self.index]
    }

    fn advance(&mut self) {
        if self.current_char() == b'\n' {
            self.line += 1;
        }
        self.index += 1;
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.index < self.buffer.len() && self.buffer[self.index] == expected {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.index < self.buffer.len() {
            match self.buffer[self.index] {
                b' ' | b'\t' | b'\r' => {
                    self.index += 1;
                }
                b'\n' => {
                    self.line += 1;
                    self.index += 1;
                }
                b'/' => {
                    if self.index + 1 < self.buffer.len() && self.buffer[self.index + 1] == b'/' {
                        // Comment till end of line
                        self.index += 2;
                        while self.index < self.buffer.len() && self.buffer[self.index] != b'\n' {
                            self.index += 1;
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn number(&mut self, start: usize) -> Option<Token> {
        while self.index < self.buffer.len() && self.buffer[self.index].is_ascii_digit() {
            self.index += 1;
        }

        // Look for a fractional part.
        if self.index < self.buffer.len() && self.buffer[self.index] == b'.' && {
            self.index + 1 < self.buffer.len() && self.buffer[self.index + 1].is_ascii_digit()
        } {
            // Consume the "."
            self.index += 1;

            while self.index < self.buffer.len() && self.buffer[self.index].is_ascii_digit() {
                self.index += 1;
            }

            // Here, you might handle exponent parts (e.g., 1e10) if needed.
        }

        Some(Token {
            tag: Tag::Number,
            loc: Loc {
                start,
                end: self.index,
            },
        })
    }

    fn identifier(&mut self, start: usize) -> Option<Token> {
        while self.index < self.buffer.len()
            && (self.buffer[self.index].is_ascii_alphanumeric() || self.buffer[self.index] == b'_')
        {
            self.index += 1;
        }

        let text = std::str::from_utf8(&self.buffer[start..self.index]).unwrap_or("");
        let tag = Tag::from_keyword(text).unwrap_or(Tag::Identifier);

        Some(Token {
            tag,
            loc: Loc {
                start,
                end: self.index,
            },
        })
    }
}

fn run_interpreter() -> io::Result<()> {
    println!("zlox interpreter v0.0.1");
    println!("Type your code below. Press Ctrl+D (Unix) or Ctrl+Z (Windows) to end input.");

    //let stdin = io::stdin();
    //let reader = stdin.lock();
    //let mut buffer = Vec::with_capacity(1 << 15);

    //// Read the entire input into buffer
    //for line_result in reader.lines() {
    //    let line = line_result?;
    //    buffer.extend_from_slice(line.as_bytes());
    //    buffer.push(b'\n'); // Retain newline characters if needed
    //}

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
                    let lexeme =
                        std::str::from_utf8(&buffer[token.loc.start..token.loc.end]).unwrap_or("");
                    println!("{:?} '{}'", token.tag, lexeme);
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
