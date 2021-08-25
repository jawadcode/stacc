use crate::{
    ast::{Expr, Literal},
    lexer::token_kind::TokenKind,
};

use super::Parser;

type ExprResult = Result<Expr, String>;

/// A trait that allows you to get the binding power of the operator `self`
trait Operator {
    fn prefix_binding_power(&self) -> Option<((), u8)>;
    fn infix_binding_power(&self) -> Option<(u8, u8)>;
}

impl Operator for TokenKind {
    fn prefix_binding_power(&self) -> Option<((), u8)> {
        Some(match self {
            TokenKind::Minus => ((), 51),
            TokenKind::Not => ((), 101),
            _ => return None,
        })
    }

    fn infix_binding_power(&self) -> Option<(u8, u8)> {
        Some(match self {
            TokenKind::Or => (1, 2),
            TokenKind::And => (3, 4),
            TokenKind::Equals | TokenKind::NotEq => (5, 6),
            TokenKind::Less | TokenKind::Greater | TokenKind::LessEq | TokenKind::GreaterEq => {
                (7, 8)
            }
            TokenKind::Plus | TokenKind::Minus => (9, 10),
            TokenKind::Multiply | TokenKind::Divide => (11, 12),
            _ => return None,
        })
    }
}

impl Parser<'_> {
    fn parse_expr(&mut self, binding_power: u8) -> ExprResult {
        let mut lhs = match self.peek() {
            TokenKind::Ident => self.parse_ident()?,
            TokenKind::Pop => self.parse_pop_expr()?,
            lit @ TokenKind::IntLit
            | lit @ TokenKind::FloatLit
            | lit @ TokenKind::StringLit
            | lit @ TokenKind::True
            | lit @ TokenKind::False => self.parse_lit(lit)?,
            TokenKind::LeftParen => self.parse_grouping()?,
            op @ TokenKind::Minus | op @ TokenKind::Not => self.parse_prefix_op(op)?,
            TokenKind::Eof => return Err("Parse error: Unexpected EOF".to_string()),
            _ => {
                let token = self.next_token().unwrap();
                return Err(self.fmt_error(
                    token.span,
                    format!("Expected expression, got {}", token.kind),
                ));
            }
        };

        loop {
            let op = match self.peek() {
                op @ TokenKind::Plus
                | op @ TokenKind::Minus
                | op @ TokenKind::Multiply
                | op @ TokenKind::Divide
                | op @ TokenKind::And
                | op @ TokenKind::Or
                | op @ TokenKind::Less
                | op @ TokenKind::Greater
                | op @ TokenKind::Not
                | op @ TokenKind::LessEq
                | op @ TokenKind::GreaterEq
                | op @ TokenKind::NotEq
                | op @ TokenKind::Equals => op,
                TokenKind::Eof | TokenKind::RightParen | TokenKind::Newline => break,
                _ => {
                    let token = self.next_token().unwrap();
                    return Err(self.fmt_error(
                        token.span,
                        format!("Expected operator or terminator, got {}", token.kind),
                    ));
                }
            };

            if let Some((left_binding_power, right_binding_power)) = op.infix_binding_power() {
                if left_binding_power < binding_power {
                    break;
                }

                self.consume(op)?;
                let rhs = self.parse_expr(right_binding_power)?;
                lhs = Expr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn parse_ident(&mut self) -> ExprResult {
        let text = {
            let token = self.next_token().unwrap();
            self.text(token)
        };

        Ok(Expr::Ident(text.to_string()))
    }

    fn parse_pop_expr(&mut self) -> ExprResult {
        self.next_token().unwrap();
        Ok(Expr::Pop)
    }

    fn parse_lit(&mut self, lit: TokenKind) -> ExprResult {
        let token = self.next_token().unwrap();
        let text = self.text(token);

        Ok(Expr::Literal(match lit {
            TokenKind::IntLit => Literal::Int(text.parse::<i64>().map_err(|_| {
                self.fmt_error(
                    token.span,
                    format!("'{}' is not a valid integer literal", text),
                )
            })?),
            TokenKind::FloatLit => Literal::Float(text.parse::<f64>().map_err(|_| {
                self.fmt_error(
                    token.span,
                    format!("'{}' is not a valid float literal", text),
                )
            })?),
            TokenKind::StringLit => Literal::String(text[1..(text.len() - 1)].to_string()),
            TokenKind::True => Literal::Bool(true),
            TokenKind::False => Literal::Bool(false),
            _ => unreachable!(),
        }))
    }

    fn parse_prefix_op(&mut self, op: TokenKind) -> ExprResult {
        self.next_token().unwrap();

        // unwrapped because it cannot fail, `op` is guaranteed to be either `not` or `-`
        let ((), right_bp) = op.prefix_binding_power().unwrap();

        let expr = Box::new(self.parse_expr(right_bp)?);
        Ok(Expr::UnaryOp { op, expr })
    }

    fn parse_grouping(&mut self) -> ExprResult {
        self.next_token().unwrap();
        let expr = self.expr();
        self.consume(TokenKind::RightParen)?;
        expr
    }

    pub fn expr(&mut self) -> ExprResult {
        self.parse_expr(0)
    }
}
