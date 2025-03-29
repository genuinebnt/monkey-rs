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
            source,
            chars: source.chars(),
        }
    }

    fn next_kind(&mut self) -> TokenKind {
        while let Some(c) = self.chars.next() {
            match c {
                '=' => return Assign,
                '+' => return Plus,
                '-' => return Minus,
                '/' => return Slash,
                '*' => return Asterisk,
                '!' => return Bang,
                '>' => return Greater,
                '<' => return Less,
                ',' => return Comma,
                ';' => return Semicolon,
                '(' => return LParen,
                ')' => return RParen,
                '{' => return LBrace,
                '}' => return RBrace,
                'a'..='z' | 'A'..='Z' | '_' => return self.read_ident(c),
                '0'..='9' => return self.read_number(),
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
        let value = &self.source[start..end];

        let value = match kind {
            Int => TokenValue::Number(value.parse::<i64>().unwrap()),
            _ => TokenValue::String(value.to_string()),
        };

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
            "true" => True,
            "false" => False,
            "if" => If,
            "else" => Else,
            "return" => Return,
            _ => Ident,
        }
    }

    fn read_number(&mut self) -> TokenKind {
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.chars.next();
            } else {
                break;
            }
        }
        TokenKind::Int
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
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}";
        let tokens = vec![
            (Let, "let"),
            (Ident, "five"),
            (Assign, "="),
            (Int, "5"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "ten"),
            (Assign, "="),
            (Int, "10"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "add"),
            (Assign, "="),
            (Function, "fn"),
            (LParen, "("),
            (Ident, "x"),
            (Comma, ","),
            (Ident, "y"),
            (RParen, ")"),
            (LBrace, "{"),
            (Ident, "x"),
            (Plus, "+"),
            (Ident, "y"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Semicolon, ";"),
            (Let, "let"),
            (Ident, "result"),
            (Assign, "="),
            (Ident, "add"),
            (LParen, "("),
            (Ident, "five"),
            (Comma, ","),
            (Ident, "ten"),
            (RParen, ")"),
            (Semicolon, ";"),
            (Bang, "!"),
            (Minus, "-"),
            (Slash, "/"),
            (Asterisk, "*"),
            (Int, "5"),
            (Semicolon, ";"),
            (Int, "5"),
            (Less, "<"),
            (Int, "10"),
            (Greater, ">"),
            (Int, "5"),
            (Semicolon, ";"),
            (If, "if"),
            (LParen, "("),
            (Int, "5"),
            (Less, "<"),
            (Int, "10"),
            (RParen, ")"),
            (LBrace, "{"),
            (Return, "return"),
            (True, "true"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Else, "else"),
            (LBrace, "{"),
            (Return, "return"),
            (False, "false"),
            (Semicolon, ";"),
            (RBrace, "}"),
            (Eof, ""),
        ];

        let mut test_cases = Vec::new();
        tokens.into_iter().for_each(|(kind, literal)| match kind {
            TokenKind::Int => {
                test_cases.push((kind, TokenValue::Number(literal.parse::<i64>().unwrap())))
            }
            _ => test_cases.push((kind, TokenValue::String(literal.to_string()))),
        });

        let mut lexer = Lexer::new(input);
        for test_case in test_cases {
            let token = lexer.next_token();
            assert_eq!(token.kind, test_case.0);
            assert_eq!(token.value, test_case.1);
        }
    }
}
