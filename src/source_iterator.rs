use peekmore::{PeekMore, PeekMoreIterator};
use std::iter::Fuse;

/// it provides lookahead for two characters at most
pub(crate) struct SourceIterator<T>
    where // TODO do I need a trait bound here on the struct?
        T: Iterator<Item=char>,
{
    source: PeekMoreIterator<Fuse<T>>,
}

impl<T> SourceIterator<T>
    where
        T: Iterator<Item=char>,
{
    pub(crate) fn new(source: T) -> Self {
        SourceIterator {
            source: source.fuse().peekmore(),
        }
    }

    /// performs a lookahead and if it matches it does use up the character
    /// and returns true
    pub(crate) fn next_if_matches(&mut self, c: char) -> bool {
        if Some(&c) == self.peek() {
            self.next();
            return true;
        }
        false
    }

    /// lookahead by one character
    pub(crate) fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    /// lookahead by two characters
    pub(crate) fn peek_ahead(&mut self) -> Option<&char> {
        self.source.peek_nth(1)
    }
}

impl<T> Iterator for SourceIterator<T>
    where
        T: Iterator<Item=char>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut source = SourceIterator::new("".chars());
        assert_eq!(source.peek(), None);
        // TODO peek one ahead
        assert_eq!(source.next(), None);
    }

    #[test]
    fn test_single_char() {
        let mut source = SourceIterator::new("A".chars());
        assert_eq!(source.peek(), Some(&'A'));
        // TODO peek one ahead
        assert_eq!(source.next(), Some('A'));
        assert_eq!(source.peek(), None);
        assert_eq!(source.next(), None);
    }

    #[test]
    fn test_two_chars() {
        let mut source = SourceIterator::new("AB".chars());
        assert_eq!(source.peek(), Some(&'A'));
        // TODO peek one ahead
        assert_eq!(source.next(), Some('A'));
        assert_eq!(source.peek(), Some(&'B'));
        // TODO peek one aheads
        assert_eq!(source.next(), Some('B'));
        assert_eq!(source.peek(), None);
        assert_eq!(source.next(), None);
    }
}