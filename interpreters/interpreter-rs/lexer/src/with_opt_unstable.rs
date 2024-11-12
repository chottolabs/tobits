// #[derive(Debug, Clone, PartialEq, Eq)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  // Add Copy
#[repr(u8)]  // Optimize enum size
pub enum Tag {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    KeywordAnd, KeywordClass, KeywordElse, KeywordFalse, KeywordFun, KeywordFor, KeywordIf, KeywordNil,
    KeywordOr, KeywordPrint, KeywordReturn, KeywordSuper, KeywordThis, KeywordTrue, KeywordVar, KeywordWhile,

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

// #[derive(Debug, Clone, Copy, PartialEq, Eq)] // Add Copy
// pub struct Loc {
//     pub start: u32, // Use u32 instead of usize for smaller size
//     pub end: u32,
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Add Copy
pub struct Loc {
    pub start: usize, // Use u32 instead of usize for smaller size
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
    len: usize,  // Cache buffer length
    line: usize, // Optional: For better error reporting
}

impl<'a> Tokenizer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Tokenizer {
            buffer,
            index: 0,
            len: buffer.len(),
            line: 1,
        }
    }

    #[inline(always)]
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.index >= self.len {
            return Some(Token {
                tag: Tag::Eof,
                loc: Loc {
                    start: self.index,
                    end: self.index,
                },
            });
        }

        let start = self.index;
        let c = self.peek()?;

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
                if matches!(self.peek(), Some(b'=')) {
                    self.advance();
                    Tag::BangEqual
                } else {
                    Tag::Bang
                }
            }
            b'=' => {
                self.advance();
                if matches!(self.peek(), Some(b'=')) {
                    self.advance();
                    Tag::EqualEqual
                } else {
                    Tag::Equal
                }
            }
            b'<' => {
                self.advance();
                if matches!(self.peek(), Some(b'=')) {
                    self.advance();
                    Tag::LessEqual
                } else {
                    Tag::Less
                }
            }
            b'>' => {
                self.advance();
                if matches!(self.peek(), Some(b'=')) {
                    self.advance();
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
                start: start,
                end: self.index,
            },
        })
    }

    #[inline(always)]
    fn advance(&mut self) {
        // if unsafe { *self.buffer.get_unchecked(self.index) } == b'\n' {
        if matches!(self.peek(), Some(b'\n')) {
            self.line += 1;
        }
        self.index += 1;
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        while self.index < self.len {
            match self.buffer[self.index] {
                b' ' | b'\r' | b'\t' => self.index += 1,
                b'\n' => {
                    self.line += 1;
                    self.index += 1;
                }
                b'/' if self.peek_next() == Some(b'/') => {
                    self.skip_line_comment();
                }
                _ => return,
            }
        }
    }

    #[inline(always)]
    fn skip_line_comment(&mut self) {
        self.index += 2;
        while let Some(c) = self.peek() {
            if c == b'\n' {
                break;
            }
            self.index += 1;
        }
    }

    #[inline(always)]
    fn number(&mut self, start: usize) -> Option<Token> {
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.index += 1;
        }

        // Look for fractional part
        if matches!(self.peek(), Some(b'.')) && matches!(self.peek_next(), Some(b'0'..=b'9')) {
            self.index += 1; // Skip the dot

            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.index += 1;
            }
        }

        Some(Token {
            tag: Tag::Number,
            loc: Loc {
                start: start,
                end: self.index,
            },
        })
    }

    #[inline(always)]
    fn peek(&self) -> Option<u8> {
        if self.index < self.len {
            Some(self.buffer[self.index])
            // Some(unsafe { *self.buffer.get_unchecked(self.index) })
        } else {
            None
        }
    }

    #[inline(always)]
    fn peek_next(&self) -> Option<u8> {
        if self.index + 1 < self.len {
            Some(self.buffer[self.index + 1])
            // Some(unsafe { *self.buffer.get_unchecked(self.index) })
        } else {
            None
        }
    }

    #[inline(always)]
    fn peek_offset(&self, offset: usize) -> Option<u8> {
        if self.index + offset < self.len {
            Some(self.buffer[self.index + offset])
        } else {
            None
        }
    }

    #[inline(always)]
    fn identifier(&mut self, start: usize) -> Option<Token> {
        while let Some(c) = self.peek() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => self.index += 1,
                _ => break,
            }
        }

        // // Use from_raw_parts for zero-copy string parsing
        // let text = unsafe {
        //     std::str::from_utf8_unchecked(std::slice::from_raw_parts(
        //         self.buffer.as_ptr().add(start),
        //         self.index - start,
        //     ))
        // };

        let text = std::str::from_utf8(&self.buffer[start..self.index]).unwrap_or("");
        let tag = Tag::from_keyword(text).unwrap_or(Tag::Identifier);
        Some(Token {
            tag,
            loc: Loc {
                start: start,
                end: self.index,
            },
        })
    }
}
