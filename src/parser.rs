use crate::lexer::{Lexer, Token, TokenKind};
use core::{fmt, panic};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Expression<'a> {
    Atom(Token<'a>),
    Operation(Token<'a>, Vec<Expression<'a>>),
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Atom(t) => write!(f, "{}", t),
            Expression::Operation(head, expressions) => {
                write!(f, "({}", head)?;
                for s in expressions {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

fn infix_binding_power(token: &TokenKind) -> (u8, u8) {
    if let TokenKind::Operator(op) = token {
        match op {
            Token::Assign => (1, 2),
            Token::Plus | Token::Minus => (3, 4),
            Token::Star | Token::Slash => (5, 6),
            _ => unreachable!(),
        }
    } else {
        panic!("expected operator token")
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Peekable<Lexer<'a>>) -> Self {
        Self { lexer }
    }
    pub fn parse(&mut self) -> Expression<'a> {
        self.parse_exp(0)
    }
    fn parse_exp(&mut self, min_bp: u8) -> Expression<'a> {
        let mut lhs = match self.lexer.next().unwrap().into_kind() {
            TokenKind::Atom(t) => Expression::Atom(t),
            TokenKind::Operator(Token::BOpen) => {
                let lhs = self.parse_exp(0);
                assert_eq!(self.lexer.next().unwrap(), Token::BClose);
                lhs
            }
            _ => panic!(),
        };
        loop {
            let kind = self.lexer.peek().unwrap().into_kind();
            let op = match kind {
                TokenKind::Atom(Token::Eof) => {
                    break;
                }

                TokenKind::Atom(Token::CbClose) => {
                    break;
                }
                TokenKind::Operator(_) => kind,
                _ => panic!(),
            };
            let (lbp, rbp) = infix_binding_power(&op);
            if lbp < min_bp {
                break;
            }
            self.lexer.next();
            let rhs = self.parse_exp(rbp);
            let TokenKind::Operator(operator) = op else {
                unreachable!()
            };
            lhs = Expression::Operation(operator, vec![lhs, rhs]);
        }
        lhs
    }
}
