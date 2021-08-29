mod token;
use token::Token;
use std::{iter::Peekable, str::Chars};

type Result<T> = std::result::Result<T, ()>;

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

    pub fn scan_tokens(&mut self) -> Vec<Result<Token>> {
        let mut vec = Vec::new();

        while let Some(c) = self.next_with_blanks_skipped() {
            if let Some(token) = self.scan_single_char(c) {
                vec.push(Ok(token));
            } else if let Some(token) = self.scan_two_chars(c) {
                vec.push(Ok(token));
                // TODO copy the lexeme into the token
                self.lexeme.clear();
            } else {
                match self.scan_multi_chars(c) {
                    Ok(Some(token)) => {
                        // TODO copy the lexeme into the token
                        vec.push(Ok(token));
                        self.lexeme.clear();
                    }
                    Ok(None) => {
                        self.lexeme.clear();
                        continue;
                    }
                    Err(err) => {
                        vec.push(Err(err));
                        self.lexeme.clear();
                    }
                }
            }
        }
        vec
    }

    fn next_with_blanks_skipped(&mut self) -> Option<char> {
        while let Some(c) = self.next() {
            match c {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                _ => return Some(c)
            };
        }
        None
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

    fn scan_multi_chars(&mut self, c: char) -> Result<Option<Token>> {
        use Token::*;
        let token = match c {
            '/' if self.next_matches('/') => {
                // the comment goes until the end of the line
                while !matches!(self.peek(), None | Some(&'\n')) {
                    self.advance();
                }
                return Ok(None);
            },
            '/' => Slash,
            _ => return Err(()),
        };
        Ok(Some(token))
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

    #[test]
    fn test_single_char() {
        let source = "+";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn test_list_single_char_tokens() {
        let source = "(){}[],.;-+/*=!><";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 17);
    }
}
