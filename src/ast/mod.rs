use std::fmt;

use crate::lexer::token_kind::TokenKind;

#[derive(Clone, Debug, PartialEq)]
/// Statement, which does not return a value and usually deals with state
pub enum Stmt {
    FnDef {
        ident: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Set {
        ident: String,
        expr: Expr,
    },
    Push(Expr),
    Print(Expr),
    FnCall(String),
    Pop,
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FnDef {
                    ident,
                    params,
                    body,
                } => format!(
                    "(define {} ({}) {})",
                    ident,
                    params.join(" "),
                    body.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
                Self::Set { ident, expr } => format!("(set {} {})", ident, expr),
                Self::Push(expr) => format!("(push {})", expr),
                Self::Print(expr) => format!("(print {})", expr),
                Self::FnCall(ident) => format!("(call {})", ident),
                Self::Pop => "pop".to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Expression, which can be evaluated
pub enum Expr {
    Ident(String),
    Literal(Literal),
    BinaryOp {
        op: TokenKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: TokenKind,
        expr: Box<Expr>,
    },
    Pop,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ident(ident) => ident.to_string(),
                Self::Literal(literal) => literal.to_string(),
                Self::BinaryOp { op, lhs, rhs } => format!("({} {} {})", op, lhs, rhs),
                Self::UnaryOp { op, expr } => format!("({} {})", op, expr),
                Self::Pop => "pop".to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Literal types
pub enum Literal {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(string) => string.to_string(),
                Self::Float(float) => float.to_string(),
                Self::Int(int) => int.to_string(),
                Self::Bool(boolean) => boolean.to_string(),
            }
        )
    }
}
