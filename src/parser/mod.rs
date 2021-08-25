pub mod expr;
pub mod stmt;

use std::iter::Peekable;

use crate::{
    ast::Stmt,
    lexer::{
        token_kind::TokenKind,
        types::{Span, Token},
        Lexer,
    },
};

/// Parser which holds the input string to extract the source text of tokens and the lexer itself
pub struct Parser<'input> {
    input: &'input str,
    lexer: Peekable<Lexer<'input>>,
}

pub enum ParseError {}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts: Vec<Stmt> = Vec::new();
        loop {
            let stmt = self.parse_stmt();
            match stmt {
                Ok(stmt) => stmts.push(stmt),
                Err(err) => {
                    if &err == "Parse error: Unexpected EOF" {
                        break;
                    }

                    return Err(err);
                }
            }
        }
        Ok(stmts)
    }

    /// Get the source text of a given token
    pub fn text(&self, token: Token) -> &'input str {
        token.text(self.input)
    }

    /// Look ahead to the next token without consuming it
    pub fn peek(&mut self) -> TokenKind {
        self.lexer
            .peek()
            .map(|token| token.kind)
            .unwrap_or(TokenKind::Eof)
    }

    /// Peek ahead to the next token and check if its `TokenKind` is `kind`
    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    /// Consume and return the next token
    pub fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    /// Consume token and check that it's `TokenKind` is as `expected`
    pub fn consume(&mut self, expected: TokenKind) -> Result<(), String> {
        let token = self.next_token().unwrap();
        if token.kind != expected {
            Err(self.fmt_error(
                token.span,
                format!("Expected {}, got {}", expected, token.kind),
            ))
        } else {
            Ok(())
        }
    }

    /// Format error with line, column and message
    pub fn fmt_error(&self, span: Span, msg: String) -> String {
        let (line, column) = span.get_line_and_column(self.input);
        format!("Parse error at {}:{} - {}", line + 1, column, msg)
    }
}
