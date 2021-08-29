#[derive(Clone, Eq, PartialEq)]
pub enum Token {
    // single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma,
    Dot,
    Minus, Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifiers, String, Number,

    // keywords
    // TODO
    EndOfFile,
}