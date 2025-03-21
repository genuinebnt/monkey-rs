#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: &str) -> Token {
        Token {
            kind,
            literal: literal.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}
