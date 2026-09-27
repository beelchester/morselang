use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token<'a> {
    Ident(&'a str),
    Number(i64),
    String(&'a str),
    Gt,
    Lt,
    Eq,
    Assign,
    Plus,
    Star,
    Minus,
    Slash,
    Set,
    If,
    Else,
    Loop,
    Function,
    BOpen,
    BClose,
    CbOpen,
    CbClose,
    Semi,
    Comma,
    Bool(bool),
    Eof,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(s) => write!(f, "Ident({s})"),
            Token::Number(n) => write!(f, "Number({n})"),
            Token::String(s) => write!(f, "String({s:?})"),
            Token::Gt => write!(f, ">"),
            Token::Lt => write!(f, "<"),
            Token::Eq => write!(f, "=="),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Star => write!(f, "*"),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Set => write!(f, "set"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Loop => write!(f, "loop"),
            Token::Function => write!(f, "fn"),
            Token::BOpen => write!(f, "("),
            Token::BClose => write!(f, ")"),
            Token::CbOpen => write!(f, "{{"),
            Token::CbClose => write!(f, "}}"),
            Token::Semi => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Bool(b) => write!(f, "{b}"),
            Token::Eof => write!(f, "<EOF>"),
        }
    }
}

pub enum TokenKind<'a> {
    Atom(Token<'a>),
    Operator(Token<'a>),
}

impl<'a> Token<'a> {
    pub fn into_kind(self) -> TokenKind<'a> {
        match self {
            Token::Gt
            | Token::Lt
            | Token::Eq
            | Token::Assign
            | Token::Plus
            | Token::Star
            | Token::Minus
            | Token::Slash => TokenKind::Operator(self),

            _ => TokenKind::Atom(self),
        }
    }
}
