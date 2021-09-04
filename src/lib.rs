mod token;
use token::Token;
use std::{iter::Peekable, str::Chars};

type Result<T> = std::result::Result<T, String>;

mod source_iterator;

use source_iterator::SourceIterator;

pub struct Scanner<T: Iterator<Item=char>> {
    source: SourceIterator<T>,
    current: Option<char>,
    line: usize,
}

impl<T> Scanner<T>
    where
        T: Iterator<Item=char>,
{
    pub fn new<'a>(source: T) -> Self {
        // TODO
        let mut source = SourceIterator::new(source);
        let first = source.next();
        Scanner {
            source,
            current: first,
            line: 0,
        }
    }

    #[deprecated]
    fn current(&self) -> Option<char> {
        self.current
    }

    #[deprecated]
    fn skip_blanks(&mut self) -> Option<char> {
        while let Some(c) = self.current() {
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

    #[deprecated]
    /// lookahead by one element
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    #[deprecated]
    fn advance(&mut self) {
        debug_assert!(!matches!(self.current(), None));
        self.source.next();
    }

    #[deprecated]
    // TODO rename advance_if_matches() or next_if_matches()
    fn next_matches(&mut self, c: char) -> bool {
        if Some(&c) == self.peek() {
            self.advance();
            return true;
        }
        false
    }

    pub fn scan_tokens(&mut self) -> Vec<Result<Token>> {
        let mut vec = Vec::new();

        while let Some(c) = self.skip_blanks() {
            if let Some(token) = self.scan_single_char(c) {
                vec.push(Ok(token));
            } else if let Some(token) = self.scan_two_chars(c) {
                vec.push(Ok(token));
            } else {
                match self.scan_multi_chars(c) {
                    Ok(Some(token)) => {
                        vec.push(Ok(token));
                    }
                    Ok(None) => {
                        continue;
                    }
                    Err(err) => {
                        vec.push(Err(err));
                    }
                }
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
        Some(token)
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
            }
            '/' => Slash,

            '"' => {
                match self.scan_string() {
                    Ok(token) => return Ok(Some(token)),
                    Err(err) => return Err(err),
                }
            }
            // no match found
            _ => return Err("no match found".to_string()),
        };
        Ok(Some(token))
    }

    fn scan_string(&mut self) -> Result<Token> /* <-- this has to be either a token or an error but not an optional */ {
        // TODO optimize by allocating the optimal capacity
        let mut lexeme = String::new();
        while !matches!(self.peek(), Some(&'"') | None) {
            if matches!(self.peek(), Some(&'\n')) {
                self.line += 1;
                self.advance();
                lexeme.push(self.current().unwrap());
            }
        }
        // either end of input or closing double quotes found
        if let None = self.current() {
            return Err("unterminated string".to_string())
        }
        self.advance(); // skip the closing double quotes
        Ok(Token::String(lexeme))
    }
}

/* #[cfg(test)]
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

    // TODO test unterminated string (EOF before closing ")
} */