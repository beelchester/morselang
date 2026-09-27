use crate::tokens::Token;

pub struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src,
            bytes: src.as_bytes(),
            cursor: 0,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    fn next_token(&mut self) -> Token<'a> {
        // whitespace skipper
        while matches!(self.peek(), Some(c) if c.is_ascii_whitespace()) {
            self.cursor += 1;
        }
        let start = self.cursor;
        let Some(current) = self.peek() else {
            return Token::Eof;
        };
        self.cursor += 1;

        // dbg!(str::from_utf8(&[current]));

        let is_morse = |b: u8| b == b'.' || b == b'-';

        let token = match current {
            b'>' => Token::Gt,
            b'<' => Token::Lt,
            b'+' => Token::Plus,
            b'-' if !matches!(self.peek(), Some(c) if is_morse(c)) => Token::Minus,
            b'*' => Token::Star,
            b'/' => Token::Slash,
            b'(' => Token::BOpen,
            b')' => Token::BClose,
            b'{' => Token::CbOpen,
            b'}' => Token::CbClose,
            b';' => Token::Semi,
            b',' => Token::Comma,
            //TODO: temp; change to morse equivalent
            b'S' => Token::Set,
            b'I' => Token::If,
            b'E' => Token::Else,
            b'L' => Token::Loop,
            b'F' => Token::Function,
            b'=' => {
                if self.peek() == Some(b'=') {
                    self.cursor += 1;
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            b if b.is_ascii_digit() => {
                while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                    self.cursor += 1;
                }
                Token::Number(
                    self.src[start..self.cursor]
                        .parse()
                        .expect("too long number"),
                )
            }
            b'"' => {
                let s_start = self.cursor;
                while !matches!(self.peek(), Some(b'"') | None) {
                    self.cursor += 1;
                }
                if self.peek().is_none() {
                    panic!("String end not found");
                }
                let body = &self.src[s_start..self.cursor];
                self.cursor += 1;
                Token::String(body)
            }
            b if is_morse(b) => {
                while matches!(self.peek(), Some(c) if is_morse(c)) {
                    self.cursor += 1;
                }
                let block = &self.src[start..self.cursor];
                match block {
                    "..." => Token::Set,
                    ".." => Token::If,
                    "." => Token::Else,
                    ".-.." => Token::Loop,
                    "..-." => Token::Function,
                    _ => morse_to_num(block).unwrap_or(Token::Ident(block)),
                }
            }
            b if b.is_ascii_alphabetic() => {
                while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric()) {
                    self.cursor += 1;
                }
                let word = &self.src[start..self.cursor];
                Token::Ident(word)
            }
            _ => unreachable!(),
        };

        token
    }
}

fn morse_to_num(morse: &str) -> Option<Token<'_>> {
    match morse {
        ".----" => Some(Token::Number(1)),
        "..---" => Some(Token::Number(2)),
        "...--" => Some(Token::Number(3)),
        "....-" => Some(Token::Number(4)),
        "....." => Some(Token::Number(5)),
        "-...." => Some(Token::Number(6)),
        "--..." => Some(Token::Number(7)),
        "---.." => Some(Token::Number(8)),
        "----." => Some(Token::Number(9)),
        "-----" => Some(Token::Number(0)),
        _ => None,
    }
}
