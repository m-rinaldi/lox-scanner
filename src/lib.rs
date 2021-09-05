mod token;
use token::Token;

type Result<T> = std::result::Result<T, String>;

mod source_iterator;

use source_iterator::SourceIterator;

pub struct Scanner<T: Iterator<Item=char>> {
    source: SourceIterator<T>,
}

impl Scanner<std::str::Chars<'_>> {
    // TODO can I do this lifetime less restrictive?
    pub fn from_str(s: &'static str) -> Self {
        let chars = s.chars();
        let source = SourceIterator::new(chars);
        Scanner {
            source: source,
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
        }
    }

    fn scan_token(&mut self) -> Result<Token> {
        while let Some(c) = self.source.next_nonblank() {
            if let Some(token) = self.scan_single_char(c) {
                return Ok(token);
            } else if let Some(token) = self.scan_two_chars(c) {
                return Ok(token);
            } else {
                match self.scan_multi_chars(c) {
                    Ok(Some(token)) => {
                        return Ok(token);
                    }

                    Ok(None) => { // comment consumed
                        continue;
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
        Ok(Token::EndOfFile)
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
            '!' if self.source.advance_if_matches('=') => BangEqual,
            '!' => Bang,

            '=' if self.source.advance_if_matches('=') => EqualEqual,
            '=' => Equal,

            '<' if self.source.advance_if_matches('=')=> LessEqual,
            '<' => Less,

            '>' if self.source.advance_if_matches('=') => GreaterEqual,
            '>' => Greater,

            _ => return None,
        };
        Some(token)
    }

    fn scan_multi_chars(&mut self, c: char) -> Result<Option<Token>> {
        use Token::*;
        let token = match c {
            '/' if self.source.advance_if_matches('/') => {
                // the comment goes until the end of the line
                while !matches!(self.source.peek(), None | Some(&'\n')) {
                    self.source.next();
                }
                return Ok(None); // comment consumed
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
        while !matches!(self.source.peek(), Some(&'"') | None) {
            // neither end of input nor end of string
            if matches!(self.source.peek(), Some(&'\n')) {
                // bypassing the next_nonblank(), soneed to keep track of new lines
                self.source.inc_current_line_by(1);
            }
            let c = self.source.next();
            lexeme.push(c.unwrap());
        }
        // either end of input or closing double quotes found
        match self.source.next() {
            None => return Err("unterminated string".to_string()),
            Some('"') => return Ok(Token::String(lexeme)),
            _ => unreachable!(),
        }
    }
}

impl<T> IntoIterator for Scanner<T>
    where
        T: Iterator<Item=char>,
{
    type Item = Result<Token>;
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
    type Item = Result<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.scanner.scan_token() {
            Ok(Token::EndOfFile) => None,
            Ok(val) => Some(Ok(val)),
            Err(err) => Some(Err(err)),
        }
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
        assert!(matches!(token, Ok(Token::EndOfFile)));
    }

    #[test]
    fn test_single_char() {
        let source = "+";
        let mut scanner = Scanner::from_str(source);
        let token = scanner.scan_token();
        assert!(matches!(token, Ok(Token::Plus)));
    }

    #[test]
    fn test_list_single_char_tokens() {
        use Token::*;
        let source = "(){}[],.;-+/*=!><";
        let mut scanner = Scanner::from_str(source);
        let mut output: Vec<Result<Token>> = vec![
            Ok(LeftParen),
            Ok(RightParen),
            Ok(LeftBrace),
            Ok(RightBrace),
            Ok(LeftBracket),
            Ok(RightBracket),
            Ok(Comma),
            Ok(Dot),
            Ok(Semicolon),
            Ok(Minus),
            Ok(Plus),
            Ok(Slash),
            Ok(Star),
            Ok(Equal),
            Ok(Bang),
            Ok(Greater),
            Ok(Less),
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
        assert!(matches!(token, Err(_)));
    }

    #[test]
    fn test_string() {
        let source = "\"FooBarBuzz\"";
        let mut scanner = Scanner::from_str(source);
        let token = scanner.scan_token();
        match token {
            Ok(Token::String(s)) => assert_eq!(s, "FooBarBuzz"),
            _ => unreachable!("it should have returned a String token"),
        }
    }
}