use std::str::Chars;

use crate::token::Token;
use crate::token::TokenKind;
use crate::token::TokenKind::*;
use crate::token::TokenValue;

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
                'a'..='z' | 'A'..='Z' | '_' => return self.read_ident(c),
                '0'..='9' => return Int,
                _ => return Illegal,
            }
        }
        TokenKind::Eof
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.offset();
        let kind = self.next_kind();
        let end = self.offset();
        let value = TokenValue::String(self.source[start..end].to_string());

        let token = Token {
            kind,
            start,
            end,
            value,
        };

        println!("{:?}", token);
        token
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn read_ident(&mut self, c: char) -> TokenKind {
        let mut ident = String::new();
        ident.push(c);
        while let Some(c) = self.peek() {
            if self.is_alphanumeric(c) {
                self.chars.next();
                ident.push(c);
            } else {
                break;
            }
        }
        self.match_ident_or_keyword(&ident)
    }

    fn match_ident_or_keyword(&self, ident: &str) -> TokenKind {
        match ident {
            "let" => Let,
            "fn" => Function,
            _ => Ident,
        }
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
            _ => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.chars.next();
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::token::TokenValue;

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);";
        let tokens = vec![
            (Let, 0, 3, "let"),
            (Ident, 4, 8, "five"),
            (Assign, 9, 10, "="),
            (Int, 11, 12, "5"),
            (Semicolon, 12, 13, ";"),
            (Let, 14, 17, "let"),
            (Ident, 18, 21, "ten"),
            (Assign, 22, 23, "="),
            (Int, 24, 25, "10"),
            (Semicolon, 25, 26, ";"),
            (Let, 28, 31, "let"),
            (Ident, 32, 35, "add"),
            (Function, 36, 40, "fn"),
            (LParen, 41, 42, "("),
            (Ident, 43, 44, "x"),
            (Comma, 45, 46, ","),
            (Ident, 47, 48, "y"),
            (RParen, 49, 50, ")"),
            (LBrace, 51, 52, "{"),
            (Ident, 54, 55, "x"),
            (Plus, 56, 57, "+"),
            (Ident, 58, 59, "y"),
            (Semicolon, 60, 61, ";"),
            (RBrace, 62, 63, "}"),
            (Semicolon, 64, 65, ";"),
            (Ident, 67, 70, "result"),
            (Assign, 71, 72, "="),
            (Ident, 74, 75, "add"),
            (LParen, 76, 77, "("),
            (Ident, 78, 79, "five"),
            (Comma, 80, 81, ","),
            (Ident, 82, 83, "ten"),
            (RParen, 84, 85, ")"),
            (Semicolon, 86, 87, ";"),
            (Eof, 88, 88, ""),
        ];

        let mut test_cases = Vec::new();
        tokens.into_iter().for_each(|(kind, start, end, literal)| {
            test_cases.push(Token::new(
                kind,
                start,
                end,
                TokenValue::String(literal.to_string()),
            ))
        });

        let mut lexer = Lexer::new(input);
        for test_case in test_cases {
            let token = lexer.next_token();
            assert_eq!(token.kind, test_case.kind);
            assert_eq!(token.start, test_case.start);
            assert_eq!(token.end, test_case.end);
            assert_eq!(token.value, test_case.value);
        }
    }
}
