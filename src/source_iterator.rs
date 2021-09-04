use std::iter::{Fuse, Peekable};

pub(crate) struct SourceIterator<T>
    where
        T: Iterator<Item=char>,
{
    source: Peekable<Fuse<T>>,
    line: usize,
}

impl<T> SourceIterator<T>
    where
        T: Iterator<Item=char>,
{
    pub(crate) fn new(source: T) -> Self {
        SourceIterator {
            source: source.fuse().peekable(),
            line: 0,
        }
    }

    /// returns the next non-blank character
    /// that is, neither carriage return, new line, tab or space
    pub(crate) fn next_nonblank(&mut self) -> Option<char> {
        while let Some(c) = self.next() {
            match c {
                ' ' | '\r' | '\t' => (),

                // update the line count
                '\n' => self.line = self.line.saturating_add(1),

                // a non-blank character
                _ => return Some(c)
            };
        }

        // input exhausted
        None
    }

    /// performs a lookahead and if it matches it does use up the character
    /// and returns true
    pub(crate) fn advance_if_matches(&mut self, c: char) -> bool {
        if Some(&c) == self.peek() {
            self.next();
            return true;
        }
        false
    }

    /// lookahead
    pub(crate) fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    pub(crate) fn current_line(&self) -> usize {
        self.line
    }

    pub(crate) fn inc_current_line_by(&mut self, inc: usize) {
        self.line = self.line.saturating_add(inc)
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
        assert_eq!(source.next(), None);
        assert_eq!(source.next_nonblank(), None);
    }
}