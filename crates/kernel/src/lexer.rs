use std::borrow::Cow;

use waken_snowball::{Algorithm, Stemmer};

pub(crate) struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
    stemmer: Stemmer,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self { source, cursor: 0, stemmer: Algorithm::English.stemmer() }
    }

    fn next_token(&mut self) -> Option<Cow<'a, str>> {
        loop {
            self.advance_while(|b| b.is_ascii_whitespace());

            let start = self.cursor;
            let byte = self.peek()?;

            self.advance();

            // only care about term start
            if byte.is_ascii_alphanumeric() {
                self.advance_while(|b| b.is_ascii_alphanumeric());

                // TODO: add a check here to filter out non alphabetic stuff???
                let term = self.slice(start);
                if term.chars().any(|c| !c.is_ascii_alphabetic()) {
                    return None;
                }

                let term = term.to_ascii_lowercase();
                return Some(self.stemmer.stem(&term).into_owned().into());
            }
        }
    }
}

impl<'a> Lexer<'a> {
    #[inline]
    fn peek(&self) -> Option<u8> { self.peek_nth(0) }

    #[inline]
    fn peek_nth(&self, n: usize) -> Option<u8> {
        self.source.as_bytes().get(self.cursor + n).copied()
    }

    #[inline]
    fn advance(&mut self) { self.cursor += 1; }

    #[inline]
    fn advance_while<P>(&mut self, predicate: P)
    where
        Self: Sized,
        P: Fn(u8) -> bool,
    {
        while self.peek().is_some_and(&predicate) {
            self.advance();
        }
    }

    fn slice(&self, start: usize) -> &'a str { &self.source[start..self.cursor] }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> { self.next_token() }
}

impl core::iter::FusedIterator for Lexer<'_> {}
