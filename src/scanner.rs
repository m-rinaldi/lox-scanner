use std::{iter::Peekable, str::Chars};

use crate::token::Token;

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

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn advance(&mut self) {
        self.source.next();
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();

        while let Some(c) = self.source.next() {
            if let Some(token) = self.scan_single_char(c) {
                vec.push(token);
            }
        }

        vec
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
        if let Some(&c) = self.peek() {
            self.advance();
            return true;
        }

        false
    }

    fn scan_two_char(&mut self, c: char) -> Option<Token> {
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
}