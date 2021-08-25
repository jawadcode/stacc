use std::{cmp, fmt};

use crate::ast::Stmt;

use self::{env::Environment, value::Value};

pub mod env;
pub mod expr;
pub mod stmt;
pub mod value;

type ValueResult = Result<Value, RuntimeError>;
type StmtResult = Result<(), RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    WrongType {
        expected: &'static str,
        got: &'static str,
    },
    UndefinedValue {
        ident: String,
    },
    CannotPerformOnType {
        op: &'static str,
        typ: &'static str,
    },
    CannotPerformOnTypeWith {
        op: &'static str,
        typ: &'static str,
        with: &'static str,
    },
    CannotCompare {
        typ: &'static str,
    },
    EmptyStack,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                &Self::WrongType { expected, got } =>
                    format!("Type error - expected {}, got {}", expected, got),
                Self::UndefinedValue { ident } => format!("Value error - {} is undefined", ident),
                &Self::CannotPerformOnType { op, typ } =>
                    format!("Type error - Cannot perform {} on {}", op, typ),
                &Self::CannotPerformOnTypeWith { op, typ, with } => format!(
                    "Type error - Cannot perform {} on {} with {}value",
                    op, typ, with
                ),
                &Self::CannotCompare { typ } =>
                    format!("Type error - Cannot perform comparison on {}", typ),
                Self::EmptyStack => "Stack error - Stack is empty".to_string(),
            }
        )
    }
}

pub struct Interpreter {
    env: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            env: Environment::default(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> StmtResult {
        for stmt in stmts {
            self.eval_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn run_one(&mut self, stmt: &Stmt) -> StmtResult {
        self.eval_stmt(stmt)
    }

    /// Why import a library when you can write 100 lines of terrible code 😎
    pub fn print_state(&self) {
        let (variables, stack) = self.env.dump();
        let mut variables = variables
            .iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect::<Vec<(&String, String)>>();
        variables.sort();

        let stack = stack
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        let stack_widest = cmp::max(stack.iter().map(String::len).max().unwrap_or(5), 5);

        let (var_widest_ident, var_widest_value) = variables
            .iter()
            .map(|(k, v)| (k.len(), v.len()))
            .fold((0, 0), |acc, elem| {
                (
                    if elem.0 > acc.0 { elem.0 } else { acc.0 },
                    if elem.1 > acc.1 { elem.1 } else { acc.1 },
                )
            });
        let var_widest_ident = cmp::max(var_widest_ident, 5);
        let var_widest_value = cmp::max(var_widest_value, 5);

        println!(
            "| Ident{} | Value{} |  | Stack{} |\n|{}|{}|  |{}|",
            " ".repeat(var_widest_ident - 5),
            " ".repeat(var_widest_value - 5),
            " ".repeat(stack_widest - 5),
            "-".repeat(var_widest_ident + 2),
            "-".repeat(var_widest_value + 2),
            "-".repeat(stack_widest + 2)
        );

        let variables_len = variables.len();
        let stack_len = stack.len();
        if variables_len > stack_len {
            for (index, (ident, value)) in variables.iter().enumerate() {
                if index < stack_len {
                    let stack_val = &stack[index];

                    let curr_ident_width = ident.len();
                    let curr_value_width = value.len();
                    let curr_stack_width = stack_val.len();

                    println!(
                        "| {}{} | {}{} |  | {}{} |",
                        ident,
                        " ".repeat(var_widest_ident - curr_ident_width),
                        value,
                        " ".repeat(var_widest_value - curr_value_width),
                        stack_val,
                        " ".repeat(stack_widest - curr_stack_width)
                    );
                } else {
                    let curr_ident_width = ident.len();
                    let curr_value_width = value.len();

                    println!(
                        "| {}{} | {}{} |",
                        ident,
                        " ".repeat(var_widest_ident - curr_ident_width),
                        value,
                        " ".repeat(var_widest_value - curr_value_width),
                    );
                }
            }
        } else {
            for (index, stack_val) in stack.iter().enumerate() {
                if index < variables_len {
                    let (ident, value) = &variables[index];

                    let curr_ident_width = ident.len();
                    let curr_value_width = value.len();
                    let curr_stack_width = stack_val.len();

                    println!(
                        "| {}{} | {}{} |  | {}{} |",
                        ident,
                        " ".repeat(var_widest_ident - curr_ident_width),
                        value,
                        " ".repeat(var_widest_value - curr_value_width),
                        stack_val,
                        " ".repeat(stack_widest - curr_stack_width)
                    );
                } else {
                    let curr_value_width = stack_val.len();

                    println!(
                        "{}| {}{} |",
                        " ".repeat(var_widest_ident + var_widest_value + 9),
                        stack_val,
                        " ".repeat(var_widest_value - curr_value_width),
                    );
                }
            }
        }
    }
}
