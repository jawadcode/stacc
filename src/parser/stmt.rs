use crate::ast::Stmt;
use crate::lexer::token_kind::TokenKind;

use super::Parser;

type StmtResult = Result<Stmt, String>;

const STMT_PREFIXES: [TokenKind; 6] = [
    TokenKind::Set,
    TokenKind::Push,
    TokenKind::Pop,
    TokenKind::Print,
    TokenKind::Begin,
    TokenKind::Call,
];

impl Parser<'_> {
    pub fn parse_stmt(&mut self) -> StmtResult {
        match self.peek() {
            TokenKind::Set => self.parse_set(),
            TokenKind::Push => self.parse_push(),
            TokenKind::Pop => self.parse_pop_stmt(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Call => self.parse_fncall(),
            TokenKind::Begin => self.parse_fndef(),
            TokenKind::Newline => self.parse_newline(),
            TokenKind::Eof => Err("Parse error: Unexpected EOF".to_string()),
            _ => {
                let token = self.next_token().unwrap();
                Err(self.fmt_error(
                    token.span,
                    format!("Expected statement, got {}", token.kind),
                ))
            }
        }
    }

    fn parse_newline(&mut self) -> StmtResult {
        while self.peek() == TokenKind::Newline {
            self.next_token().unwrap();
        }
        self.parse_stmt()
    }

    #[inline]
    fn is_statement(&mut self) -> bool {
        STMT_PREFIXES.contains(&self.peek())
    }

    fn ident(&mut self) -> Result<String, String> {
        let ident = self.next_token().unwrap();
        if let TokenKind::Ident = ident.kind {
            Ok(self.text(ident).to_string())
        } else {
            Err(self.fmt_error(
                ident.span,
                format!("Expected identifier, got {}", ident.kind),
            ))
        }
    }

    fn parse_set(&mut self) -> StmtResult {
        self.next_token().unwrap();

        let text = self.ident()?;
        let expr = self.expr()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Set { ident: text, expr })
    }

    fn parse_push(&mut self) -> StmtResult {
        self.next_token().unwrap();
        let expr = self.expr()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Push(expr))
    }

    fn parse_pop_stmt(&mut self) -> StmtResult {
        self.next_token().unwrap();
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Pop)
    }

    fn parse_print(&mut self) -> StmtResult {
        self.next_token().unwrap();
        let expr = self.expr()?;
        self.consume(TokenKind::Newline)?;
        Ok(Stmt::Print(expr))
    }

    fn parse_fncall(&mut self) -> StmtResult {
        self.next_token().unwrap();
        let ident = self.next_token().unwrap();
        match ident.kind {
            TokenKind::Ident => (),
            _ => {
                return Err(self.fmt_error(
                    ident.span,
                    format!("Expected function identifier, got {}", ident.kind),
                ))
            }
        }

        let text = self.text(ident).to_string();
        Ok(Stmt::FnCall(text))
    }

    fn parse_fndef(&mut self) -> StmtResult {
        self.next_token().unwrap();
        let ident = self.next_token().unwrap();
        match ident.kind {
            TokenKind::Ident => (),
            _ => {
                return Err(self.fmt_error(
                    ident.span,
                    format!("Expected identifier, got {}", ident.kind),
                ))
            }
        }

        let ident = self.text(ident).to_string();
        self.consume(TokenKind::Colon)?;

        let mut params = Vec::new();
        while self.at(TokenKind::Ident) {
            params.push({
                let token = self.next_token().unwrap();
                self.text(token).to_string()
            });
        }
        self.consume(TokenKind::Newline)?;

        let mut body = Vec::new();
        loop {
            let stmt = self.parse_stmt()?;
            body.push(stmt);

            if let TokenKind::End = self.peek() {
                break;
            }

            if !self.is_statement() {
                let token = self.next_token().unwrap();
                return Err(self.fmt_error(
                    token.span,
                    format!("Expected statement or 'end', got {}", token.kind),
                ));
            }
        }
        self.consume(TokenKind::End)?;

        Ok(Stmt::FnDef {
            ident,
            params,
            body,
        })
    }
}
