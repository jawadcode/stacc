use crate::ast::{Expr, Stmt};

use super::{
    value::{Function, Value},
    Interpreter, StmtResult,
};

impl Interpreter {
    pub fn eval_stmt(&mut self, stmt: &Stmt) -> StmtResult {
        match stmt {
            Stmt::FnDef {
                ident,
                params,
                body,
            } => self.eval_fndef(ident, params, body),
            Stmt::Set { ident, expr } => self.eval_set(ident, expr),
            Stmt::Push(expr) => self.eval_push(expr),
            Stmt::Print(expr) => self.eval_print(expr),
            Stmt::FnCall(ident) => self.eval_fncall(ident),
            Stmt::Pop => self.eval_pop(),
        }
    }

    fn eval_fndef(&mut self, ident: &str, params: &[String], body: &[Stmt]) -> StmtResult {
        let function = Function {
            ident: ident.to_string(),
            params: params.to_vec(),
            body: body.to_vec(),
        };
        self.env.set(ident.to_string(), Value::Function(function));
        Ok(())
    }

    fn eval_set(&mut self, ident: &str, expr: &Expr) -> StmtResult {
        let expr = self.eval_expr(expr)?;
        self.env.set(ident.to_string(), expr);
        Ok(())
    }

    fn eval_push(&mut self, expr: &Expr) -> StmtResult {
        let expr = self.eval_expr(expr)?;
        self.env.push(expr);
        Ok(())
    }

    fn eval_print(&mut self, expr: &Expr) -> StmtResult {
        let expr = self.eval_expr(expr)?;
        println!("{}", expr);
        Ok(())
    }

    fn eval_fncall(&mut self, ident: &str) -> StmtResult {
        let function = self.env.get(ident)?;
        let function = function.to_function()?;

        self.env.new_scope();
        for param in &function.params {
            let popped = self.env.parent_pop()?;
            self.env.set(param.to_string(), popped);
        }
        for stmt in &function.body {
            self.eval_stmt(stmt)?;
        }
        let return_value = self.env.pop();
        self.env.exit_scope();
        
        if let Ok(value) = return_value {
            self.env.push(value)
        }

        Ok(())
    }

    fn eval_pop(&mut self) -> StmtResult {
        self.env.pop().map(|_| ())
    }
}
