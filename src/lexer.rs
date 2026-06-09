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
    input: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self { Self { input, cursor: 0 } }

    #[inline]
    fn peek(&self) -> Option<u8> { self.input.as_bytes().get(self.cursor).copied() }

    fn advance(&mut self) { self.cursor += 1; }

    fn advance_while(&mut self, mut predicate: impl FnMut(u8) -> bool) {
        while let Some(b) = self.peek() {
            if !predicate(b) {
                break;
            }

            self.advance();
        }
    }

    fn span(&self, start: usize) -> Span { Span { start, end: self.cursor } }

    fn slice(&self, start: usize) -> &'a str { &self.input[start..self.cursor] }

    fn next_token(&mut self) -> Option<Token<'a>> {
        loop {
            self.advance_while(|b| b.is_ascii_whitespace());

            let start = self.cursor;
            let byte = self.peek()?;

            // only care about term start
            if byte.is_ascii_alphanumeric() {
                self.advance();

                self.advance_while(|b| b.is_ascii_alphanumeric());

                let lexeme = self.slice(start);
                return Some(Token { span: self.span(start), lexeme });
            }

            // skip everything else
            self.advance();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> { self.next_token() }
}
