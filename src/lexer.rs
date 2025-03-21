use std::str::Chars;

use crate::token::Token;
use crate::token::TokenKind;
use crate::token::TokenKind::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source,
            chars: source.chars(),
        }
    }

    fn next_kind(&mut self) -> TokenKind {
        while let Some(c) = self.chars.next() {
            match c {
                '=' => return Assign,
                '+' => return Plus,
                ',' => return Comma,
                ';' => return Semicolon,
                '(' => return LParen,
                ')' => return RParen,
                '{' => return LBrace,
                '}' => return RBrace,
                'a'..='z' | 'A'..='Z' => return Ident,
                '0'..='9' => return Int,
                _ => return Illegal,
            }
        }
        TokenKind::Eof
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.next_kind();
        let end = self.offset();
        Token {
            kind,
            literal: self.source[start..end].to_string(),
        }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tokens = vec![
            (Assign, "="),
            (Plus, "+"),
            (LParen, "("),
            (RParen, ")"),
            (LBrace, "{"),
            (RBrace, "}"),
            (Comma, ","),
            (Semicolon, ";"),
        ];

        let mut test_cases = Vec::new();
        tokens
            .into_iter()
            .for_each(|(kind, literal)| test_cases.push(Token::new(kind, literal)));

        let mut lexer = Lexer::new(input);
        for test_case in test_cases {
            let token = lexer.next_token();
            assert_eq!(token.kind, test_case.kind);
            assert_eq!(token.literal, test_case.literal);
        }
    }
}
