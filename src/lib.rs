mod token;
mod source_iterator;

use source_iterator::SourceIterator;
use token::Token;

pub struct Scanner<T: Iterator<Item=char>> {
    source: SourceIterator<T>,
    line: usize,
    // TODO keep track of the column as well
}

impl Scanner<std::str::Chars<'_>> {
    // TODO can I do this with a less restrictive lifetime?
    pub fn from_str(s: &'static str) -> Self {
        let chars = s.chars();
        let source = SourceIterator::new(chars);
        Scanner {
            source: source,
            line: 0,
        }
    }
}

impl<T> Scanner<T>
    where
        T: Iterator<Item=char>,
{
    // TODO implement from iterator instead
    pub fn new(source: T) -> Self {
        let source = SourceIterator::new(source);
        Scanner {
            source,
            line: 0,
        }
    }

    pub fn next_nonblank(&mut self) -> Option<char> {
        while let Some(c) = self.source.next() {
            match c {
                ' ' | '\r' | '\t' => (),
                // update the line count
                '\n' => self.line = self.line.saturating_add(1),
                _ => return Some(c),
            }
        }
        None
    }

    fn scan_token(&mut self) -> Option<Token> {
        while let Some(c) = self.next_nonblank() {
            if let Some(token) = self.scan_single_char(c) {
                return Some(token);
            } else if let Some(token) = self.scan_two_chars(c) {
                return Some(token);
            } else if let Some(token) = self.scan_multi_chars(c) {
                return Some(token);
            }
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
            '[' => LeftBracket,
            ']' => RightBracket,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            '*' => Star,
            ';' => Semicolon,
            '=' => Equal,
            _ => return None,
        };
        Some(token)
    }

    fn scan_two_chars(&mut self, c: char) -> Option<Token> {
        use Token::*;
        let token = match c {
            '!' if self.source.next_if_matches('=') => BangEqual,
            '!' => Bang,

            '=' if self.source.next_if_matches('=') => EqualEqual,
            '=' => Equal,

            '<' if self.source.next_if_matches('=')=> LessEqual,
            '<' => Less,

            '>' if self.source.next_if_matches('=') => GreaterEqual,
            '>' => Greater,

            _ => return None,
        };
        Some(token)
    }

    fn scan_multi_chars(&mut self, c: char) -> Option<Token> {
        use Token::*;
        match c {
            '/' if self.source.next_if_matches('/') => {
                // the comment goes until the end of the line
                while !matches!(self.source.peek(), None | Some(&'\n')) {
                    self.source.next();
                }
                None // comment consumed
            }
            '/' => Some(Slash),
            '"' => return self.scan_string(),
            // no match found
            _ => Some(Invalid("no match found".to_string(), self.line)),
        }
    }

    fn scan_string(&mut self) -> Option<Token> /* <-- this has to be either a token or an error but not an optional */ {
        // TODO optimize by allocating the optimal capacity
        let mut lexeme = String::new();
        while !matches!(self.source.peek(), Some(&'"') | None) {
            // neither end of input nor end of string
            if matches!(self.source.peek(), Some(&'\n')) {
                // bypassing the next_nonblank(), soneed to keep track of new lines
                self.line = self.line.saturating_add(1);
            }
            let c = self.source.next();
            lexeme.push(c.unwrap());
        }
        // either end of input or closing double quotes found
        match self.source.next() {
            None => return Some(Token::Invalid("unterminated string".to_string(), self.line)),
            Some('"') => return Some(Token::String(lexeme)),
            _ => unreachable!(),
        }
    }
}

impl<T> IntoIterator for Scanner<T>
    where
        T: Iterator<Item=char>,
{
    type Item = Token;
    type IntoIter = TokenIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        TokenIterator {
            scanner: self
        }
    }
}

pub struct TokenIterator<T: Iterator<Item=char>> {
    scanner: Scanner<T>,
}

impl<T> Iterator for TokenIterator<T>
    where T:
        Iterator<Item=char>,
{
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.scan_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let source = "";
        let mut scanner = Scanner::from_str(source);
        let token = scanner.scan_token();
        assert!(matches!(token, None));
    }

    #[test]
    fn test_single_char() {
        let source = "+";
        let mut scanner = Scanner::from_str(source);
        let token = scanner.scan_token();
        assert!(matches!(token, Some(Token::Plus)));
    }

    #[test]
    fn test_list_single_char_tokens() {
        use Token::*;
        let source = "(){}[],.;-+/*=!><";
        let scanner = Scanner::from_str(source);
        let mut output = vec![
            LeftParen,
            RightParen,
            LeftBrace,
            RightBrace,
            LeftBracket,
            RightBracket,
            Comma,
            Dot,
            Semicolon,
            Minus,
            Plus,
            Slash,
            Star,
            Equal,
            Bang,
            Greater,
            Less,
        ];

        output.reverse();

        for token in scanner {
            assert_eq!(token, output.pop().unwrap());
        }
    }

    #[test]
    fn test_unterminated_string() {
        let source = "\"this is unterminated\nstring";
        let mut scanner = Scanner::from_str(source);
        let token = scanner.scan_token();
        assert!(matches!(token, Some(Token::Invalid(_,_))));
    }

    #[test]
    fn test_string() {
        let source = "\"FooBarBuzz\"";
        let mut scanner = Scanner::from_str(source);
        let token = scanner.scan_token();
        match token {
            Some(Token::String(s)) => assert_eq!(s, "FooBarBuzz"),
            _ => unreachable!("it should have returned a String token"),
        }
    }
}