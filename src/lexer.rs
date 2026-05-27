pub(crate) struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self { Self { input, cursor: 0 } }

    #[inline]
    fn remaining(&self) -> &'a [u8] { self.input.as_bytes().get(self.cursor..).unwrap_or_default() }

    #[inline]
    fn peek(&self) -> Option<u8> { self.remaining().first().copied() }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|b| b.is_ascii_whitespace()) {
            self.cursor += 1;
        }
    }

    fn advance_while(&mut self, mut predicate: impl FnMut(u8) -> bool) {
        while self.peek().is_some_and(&mut predicate) {
            self.cursor += 1;
        }
    }

    fn slice(&self, start: usize) -> &'a str {
        // SAFETY: all advances happen at ASCII character boundaries,
        // which are always valid UTF-8 boundaries.
        &self.input[start..self.cursor]
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        loop {
            self.skip_whitespace();

            let start = self.cursor;
            let b = self.peek()?;
            self.cursor += 1;

            match b {
                b'0'..=b'9' | b'.' => return Some(self.lex_number(start)),
                b'a'..=b'z' | b'A'..=b'Z' => return Some(self.lex_ident(start)),
                _ => {}
            }
        }
    }

    fn lex_number(&mut self, start: usize) -> Token<'a> {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'.');
        let s = self.slice(start);
        let n = s.parse().unwrap_or_default();
        Token::Num(n)
    }

    fn lex_ident(&mut self, start: usize) -> Token<'a> {
        self.advance_while(|b| b.is_ascii_alphanumeric());
        let s = self.slice(start);
        Token::Str(s)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> { self.next_token() }
}

#[derive(Debug)]
pub(crate) enum Token<'a> {
    Num(f64),
    Str(&'a str),
}
