#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot, Semicolon,
    Minus, Plus,
    Slash, Star,

    // one-character tokens and two-character tokens that contain
    // these one-character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifiers(String), String(String), Number(String, f32),

    // keywords
    // TODO
    EndOfFile,
}