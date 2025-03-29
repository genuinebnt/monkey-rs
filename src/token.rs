use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize, value: TokenValue) -> Token {
        Token {
            kind,
            start,
            end,
            value,
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
    Minus,
    Slash,
    Asterisk,
    Bang,
    Greater,
    Less,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "Assign"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Slash => write!(f, "Slash"),
            TokenKind::Asterisk => write!(f, "Asterisk"),
            TokenKind::Bang => write!(f, "Bang"),
            TokenKind::Greater => write!(f, "Greater"),
            TokenKind::Less => write!(f, "Less"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::RParen => write!(f, "RParen"),
            TokenKind::LBrace => write!(f, "LBrace"),
            TokenKind::RBrace => write!(f, "RBrace"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::True => write!(f, "True"),
            TokenKind::False => write!(f, "False"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::Return => write!(f, "Return"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    None,
    Number(i64),
    String(String),
}
