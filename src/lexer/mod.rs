pub mod token_kind;
pub mod types;

use token_kind::*;
use types::*;

use logos::Logos;

pub struct Lexer<'input> {
    input: &'input str,
    generated: logos::SpannedIter<'input, LogosToken>,
    eof: bool,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input: input,
            generated: LogosToken::lexer(input).spanned(),
            eof: false,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generated.next() {
            Some((token, span)) => Some(Token {
                kind: TokenKind::from(token),
                span: span.into(),
            }),
            None if self.eof => None,
            None => {
                self.eof = true;
                Some(Token {
                    kind: TokenKind::Eof,
                    span: (self.input.len() - 1..self.input.len() - 1).into(),
                })
            }
        }
    }
}
