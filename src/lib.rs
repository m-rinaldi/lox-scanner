mod token;
use token::Token;
use std::{iter::Peekable, str::Chars};

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    lexeme: String,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let source = source.chars().peekable();
        Scanner {
            source,
            lexeme: String::new(),
            line: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.source.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn advance(&mut self) {
        self.next();
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();
        while let Some(c) = self.next_skip_blanks() {
            if let Some(token) = self.scan_single_char(c) {
                vec.push(token);
            } else if let Some(token) = self.scan_two_chars(c) {
                vec.push(token);
                // TODO copy the lexeme into the token
                self.lexeme.clear();
            } else if let Some(token) = self.scan_multi_chars(c) {
                // TODO copy the lexeme into the token
                vec.push(token);
                self.lexeme.clear();
            }
        }
        vec
    }

    fn next_skip_blanks(&mut self) -> Option<char> {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                _ => (),
            };
        }
        self.next()
    }

    fn scan_single_char(&mut self, c: char) -> Option<Token> {
        use Token::*;
        let token = match c {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            ';' => Semicolon,
            '=' => Star,
            _ => return None,
        };

        self.lexeme.clear();
        Some(token)
    }

    fn next_matches(&mut self, c: char) -> bool {
        if Some(&c) == self.peek() {
            self.advance();
            return true;
        }

        false
    }

    fn scan_two_chars(&mut self, c: char) -> Option<Token> {
        use Token::*;
        let token = match c {
            '!' if self.next_matches('=') => BangEqual,
            '!' => Bang,

            '=' if self.next_matches('=') => EqualEqual,
            '=' => Equal,

            '<' if self.next_matches('=')=> LessEqual,
            '<' => Less,

            '>' if self.next_matches('=') => GreaterEqual,
            '>' => GreaterEqual,

            _ => return None,
        };

        Some(token)
    }

    fn scan_multi_chars(&mut self, c: char) -> Option<Token> {
        use Token::*;
        let token = match c {
            '/' if self.next_matches('/') => {
                // the comment goes until the end of the line
                while !matches!(self.peek(), None | Some(&'\n')) {
                    self.advance();
                }
                return None;
            },

            '/' => Slash,

            _ => return None,
        };
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let source = "";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert!(tokens.is_empty());
    }

    fn test_single_char() {
        let source = "+";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
    }

    #[test]
    fn test_single_char_token() {
        let source = "(){}[],.;-+/*=!><";

    }
}
