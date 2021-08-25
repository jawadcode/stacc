use std::fmt;

use crate::ast::Stmt;

use super::{RuntimeError, ValueResult};

#[derive(Clone, Debug)]
pub enum Value {
    Function(Function),
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub ident: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Function(_) => "function",
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Bool(_) => "boolean",
        }
    }

    pub fn to_function(&self) -> Result<&Function, RuntimeError> {
        match self {
            Value::Function(function) => Ok(function),
            _ => Err(RuntimeError::WrongType {
                expected: "function",
                got: self.type_name(),
            }),
        }
    }

    pub fn to_str(&self) -> Result<&str, RuntimeError> {
        match self {
            Value::String(string) => Ok(string),
            _ => Err(RuntimeError::WrongType {
                expected: "string",
                got: self.type_name(),
            }),
        }
    }

    pub fn to_number(&self) -> Result<f64, RuntimeError> {
        match self {
            Value::Number(number) => Ok(*number),
            _ => Err(RuntimeError::WrongType {
                expected: "number",
                got: self.type_name(),
            }),
        }
    }

    pub fn add(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotPerformOnType {
                op: "addition",
                typ: "function",
            }),
            Value::String(string) => Ok(Value::String(string.to_string() + other.to_str()?)),
            Value::Number(number) => Ok(Value::Number(number + other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotPerformOnType {
                op: "addition",
                typ: "boolean",
            }),
        }
    }

    pub fn sub(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotPerformOnType {
                op: "subtraction",
                typ: "function",
            }),
            Value::String(_) => Err(RuntimeError::CannotPerformOnType {
                op: "subtraction",
                typ: "string",
            }),
            Value::Number(number) => Ok(Value::Number(number - other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotPerformOnType {
                op: "subtraction",
                typ: "booelan",
            }),
        }
    }

    pub fn mul(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotPerformOnType {
                op: "multiplication",
                typ: "function",
            }),
            Value::String(string) => Ok(Value::String(match other {
                Value::Function(_) => {
                    return Err(RuntimeError::CannotPerformOnTypeWith {
                        op: "multiplication",
                        typ: "string",
                        with: "function",
                    })
                }
                Value::String(_) => {
                    return Err(RuntimeError::CannotPerformOnTypeWith {
                        op: "multiplication",
                        typ: "string",
                        with: "string",
                    })
                }
                Value::Number(number) => string.repeat(number as usize),
                Value::Bool(_) => {
                    return Err(RuntimeError::CannotPerformOnTypeWith {
                        op: "multiplication",
                        typ: "string",
                        with: "boolean",
                    })
                }
            })),
            Value::Number(number) => Ok(Value::Number(number * other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotPerformOnType {
                op: "multiplication",
                typ: "boolean",
            }),
        }
    }

    pub fn div(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotPerformOnType {
                op: "division",
                typ: "function",
            }),
            Value::String(_) => Err(RuntimeError::CannotPerformOnType {
                op: "division",
                typ: "string",
            }),
            Value::Number(number) => Ok(Value::Number(number / other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotPerformOnType {
                op: "division",
                typ: "boolean",
            }),
        }
    }

    pub fn lt(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotCompare { typ: "function" }),
            Value::String(string) => Ok(Value::Bool(string.as_str() < other.to_str()?)),
            Value::Number(number) => Ok(Value::Bool(*number < other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotCompare { typ: "boolean" }),
        }
    }

    pub fn gt(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotCompare { typ: "function" }),
            Value::String(string) => Ok(Value::Bool(string.as_str() > other.to_str()?)),
            Value::Number(number) => Ok(Value::Bool(*number > other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotCompare { typ: "boolean" }),
        }
    }

    pub fn le(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotCompare { typ: "function" }),
            Value::String(string) => Ok(Value::Bool(string.as_str() <= other.to_str()?)),
            Value::Number(number) => Ok(Value::Bool(*number <= other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotCompare { typ: "boolean" }),
        }
    }

    pub fn ge(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotCompare { typ: "function" }),
            Value::String(string) => Ok(Value::Bool(string.as_str() >= other.to_str()?)),
            Value::Number(number) => Ok(Value::Bool(*number >= other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotCompare { typ: "boolean" }),
        }
    }

    pub fn eq(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotCompare { typ: "function" }),
            Value::String(string) => Ok(Value::Bool(string == other.to_str()?)),
            Value::Number(number) => Ok(Value::Bool(*number == other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotCompare { typ: "boolean" }),
        }
    }

    pub fn ne(&self, other: Value) -> ValueResult {
        match self {
            Value::Function(_) => Err(RuntimeError::CannotCompare { typ: "function" }),
            Value::String(string) => Ok(Value::Bool(string != other.to_str()?)),
            Value::Number(number) => Ok(Value::Bool(*number != other.to_number()?)),
            Value::Bool(_) => Err(RuntimeError::CannotCompare { typ: "boolean" }),
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Function(function) => !function.body.is_empty(),
            Value::String(string) => !string.is_empty(),
            Value::Number(number) => number == 0.0,
            Value::Bool(boolean) => boolean,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Function(function) => format!(
                    "<function {}({})>",
                    function.ident,
                    function.params.join(", ")
                ),
                Value::String(string) => string.to_string(),
                Value::Number(number) => number.to_string(),
                Value::Bool(boolean) => boolean.to_string(),
            }
        )
    }
}
