#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Token<'a> {
    pub span: Span,
    pub lexeme: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Span {
    pub start: usize,
    pub end: usize,
}

pub(crate) struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(source: &'a str) -> Self { Self { source, cursor: 0 } }

    fn next_token(&mut self) -> Option<Token<'a>> {
        loop {
            self.advance_while(|b| b.is_ascii_whitespace());

            let start = self.cursor;
            let byte = self.peek()?;

            self.advance();

            // only care about term start
            if byte.is_ascii_alphanumeric() {
                self.advance_while(|b| b.is_ascii_alphanumeric());

                let lexeme = self.slice(start);
                return Some(Token { span: self.span(start), lexeme });
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

    fn span(&self, start: usize) -> Span { Span { start, end: self.cursor } }

    fn slice(&self, start: usize) -> &'a str { &self.source[start..self.cursor] }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> { self.next_token() }
}

impl core::iter::FusedIterator for Lexer<'_> {}
