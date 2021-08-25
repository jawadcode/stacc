use crate::{
    ast::{Expr, Literal},
    lexer::token_kind::TokenKind,
};

use super::{value::Value, Interpreter, ValueResult};

impl Interpreter {
    pub fn eval_expr(&mut self, expr: &Expr) -> ValueResult {
        match expr {
            Expr::Ident(ident) => self.env.get(ident),
            Expr::Literal(literal) => Ok(Self::eval_literal(literal)),
            Expr::BinaryOp { op, lhs, rhs } => self.eval_binary_op(op, lhs, rhs),
            Expr::UnaryOp { op, expr } => self.eval_unary_op(op, expr),
            Expr::Pop => self.env.pop(),
        }
    }

    fn eval_literal(literal: &Literal) -> Value {
        match literal {
            Literal::Int(int) => Value::Number(*int as f64),
            Literal::Float(float) => Value::Number(*float),
            Literal::String(string) => Value::String(string.clone()),
            Literal::Bool(boolean) => Value::Bool(*boolean),
        }
    }

    fn eval_binary_op(&mut self, op: &TokenKind, lhs: &Expr, rhs: &Expr) -> ValueResult {
        let lhs = self.eval_expr(lhs)?;
        match op {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Multiply
            | TokenKind::Divide
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEq
            | TokenKind::GreaterEq
            | TokenKind::NotEq
            | TokenKind::Equals => self.eval_binary_op_numerical(op, lhs, rhs),
            TokenKind::And | TokenKind::Or => self.eval_binary_op_short_circuiting(op, lhs, rhs),
            _ => unreachable!(),
        }
    }

    fn eval_binary_op_numerical(&mut self, op: &TokenKind, lhs: Value, rhs: &Expr) -> ValueResult {
        let rhs = self.eval_expr(rhs)?;
        match op {
            TokenKind::Plus => lhs.add(rhs),
            TokenKind::Minus => lhs.sub(rhs),
            TokenKind::Multiply => lhs.mul(rhs),
            TokenKind::Divide => lhs.div(rhs),
            TokenKind::Less => lhs.lt(rhs),
            TokenKind::Greater => lhs.gt(rhs),
            TokenKind::LessEq => lhs.le(rhs),
            TokenKind::GreaterEq => lhs.ge(rhs),
            TokenKind::NotEq => lhs.ne(rhs),
            TokenKind::Equals => lhs.eq(rhs),
            _ => unreachable!(),
        }
    }

    fn eval_binary_op_short_circuiting(
        &mut self,
        op: &TokenKind,
        lhs: Value,
        rhs: &Expr,
    ) -> ValueResult {
        Ok(match op {
            TokenKind::And => Value::Bool(bool::from(lhs) && bool::from(self.eval_expr(rhs)?)),
            TokenKind::Or => Value::Bool(bool::from(lhs) || bool::from(self.eval_expr(rhs)?)),
            _ => unreachable!(),
        })
    }

    fn eval_unary_op(&mut self, op: &TokenKind, expr: &Expr) -> ValueResult {
        let expr = self.eval_expr(expr)?;
        Ok(match op {
            TokenKind::Minus => Value::Number(-expr.to_number()?),
            TokenKind::Not => Value::Bool(!bool::from(expr)),
            _ => unreachable!(),
        })
    }
}
