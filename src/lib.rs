mod token;
use token::Token;

type Result<T> = std::result::Result<T, String>;

mod source_iterator;

use source_iterator::SourceIterator;

pub struct Scanner<T: Iterator<Item=char>> {
    source: SourceIterator<T>,
}

impl<T> Scanner<T>
    where
        T: Iterator<Item=char>,
{
    pub fn new<'a>(source: T) -> Self {
        let source = SourceIterator::new(source);
        Scanner {
            source,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Result<Token>> {
        let mut vec = Vec::new();

        while let Some(c) = self.source.next_nonblank() {
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
            '!' if self.source.advance_if_matches('=') => BangEqual,
            '!' => Bang,

            '=' if self.source.advance_if_matches('=') => EqualEqual,
            '=' => Equal,

            '<' if self.source.advance_if_matches('=')=> LessEqual,
            '<' => Less,

            '>' if self.source.advance_if_matches('=') => GreaterEqual,
            '>' => GreaterEqual,

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
        while !matches!(self.source.peek(), Some(&'"') | None) {
            // neither end of input nor end of string
            if matches!(self.source.peek(), Some(&'\n')) {
                // bypassing the next_nonblank(), soneed to keep track of new lines
                self.source.inc_current_line_by(1);
                let c = self.source.next();
                lexeme.push(c.unwrap());
            }
        }
        // either end of input or closing double quotes found
        match self.source.next() {
            None => return Err("unterminated string".to_string()),
            Some('"') => return Ok(Token::String(lexeme)),
            _ => unreachable!(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_empty_source() {
//         let source = "";
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();
//         assert!(tokens.is_empty());
//     }

//     #[test]
//     fn test_single_char() {
//         let source = "+";
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();
//         assert_eq!(tokens.len(), 1);
//     }

//     #[test]
//     fn test_list_single_char_tokens() {
//         let source = "(){}[],.;-+/*=!><";
//         let mut scanner = Scanner::new(source);
//         let tokens = scanner.scan_tokens();
//         assert_eq!(tokens.len(), 17);
//     }

//     // TODO test unterminated string (EOF before closing ")
// }