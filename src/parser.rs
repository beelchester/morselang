use crate::lexer::Lexer;
use crate::tokens::{Token, TokenKind};
use core::{fmt, panic};
use std::iter::Peekable;

#[derive(Debug)]
pub enum TokenTree<'a> {
    Atom(Token<'a>),
    Cons(Token<'a>, Vec<TokenTree<'a>>),
}

impl<'a> fmt::Display for TokenTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenTree::Atom(t) => write!(f, "{}", t),
            TokenTree::Cons(head, expressions) => {
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
    pub fn parse(&mut self) -> TokenTree<'a> {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> TokenTree<'a> {
        let token = self.lexer.next().unwrap();
        dbg!(&token);
        match token {
            Token::Set => {
                let identifier = TokenTree::Atom(self.lexer.next().unwrap());
                if !matches!(self.lexer.next().unwrap(), Token::Assign) {
                    panic!("expected assign")
                };
                let rhs = self.parse_exp(0);
                if !matches!(self.lexer.next().unwrap(), Token::Semi) {
                    panic!("expected semicolan")
                };
                TokenTree::Cons(Token::Set, vec![identifier, rhs])
            }
            Token::Function => {
                let identifier = TokenTree::Atom(self.lexer.next().unwrap());
                if !matches!(self.lexer.next().unwrap(), Token::BOpen) {
                    panic!("expected (")
                };
                if !matches!(self.lexer.next().unwrap(), Token::BClose) {
                    panic!("expected )")
                };
                if !matches!(self.lexer.next().unwrap(), Token::CbOpen) {
                    panic!("expected {{");
                };

                let block = self.parse_statement();

                if !matches!(self.lexer.next().unwrap(), Token::CbClose) {
                    panic!("expected }}");
                };

                TokenTree::Cons(Token::Function, vec![identifier, block])
            }
            t => todo!("{t}"),
        }
    }

    fn parse_exp(&mut self, min_bp: u8) -> TokenTree<'a> {
        let mut lhs = match self.lexer.next().unwrap().into_kind() {
            TokenKind::Atom(t) => TokenTree::Atom(t),
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

                TokenKind::Atom(Token::BClose) => {
                    break;
                }
                TokenKind::Operator(_) => kind,
                TokenKind::Atom(Token::Semi) => {
                    break;
                }
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
            lhs = TokenTree::Cons(operator, vec![lhs, rhs]);
        }
        lhs
    }
}
