use std::collections::HashMap;

use super::{value::Value, RuntimeError, ValueResult};

#[derive(Debug)]
pub struct Environment {
    variables: Vec<HashMap<String, Value>>,
    stack: Vec<Vec<Value>>,
    depth: usize,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            variables: vec![HashMap::new()],
            stack: vec![Vec::new()],
            depth: 0,
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_scope(&mut self) {
        self.variables.push(HashMap::new());
        self.stack.push(Vec::new());
        self.depth += 1;
    }

    pub fn exit_scope(&mut self) {
        self.variables.pop();
        self.stack.pop();
        self.depth -= 1;
    }

    #[inline]
    pub fn get(&self, name: &str) -> ValueResult {
        for i in (0..=self.depth).rev() {
            match self.variables[i].get(name).map(Clone::clone) {
                Some(value) => return Ok(value),
                None => (),
            }
        }

        Err(RuntimeError::UndefinedValue {
            ident: name.to_string(),
        })
    }

    #[inline]
    pub fn set(&mut self, name: String, value: Value) {
        self.variables
            .get_mut(self.depth)
            .unwrap()
            .insert(name, value);
    }

    #[inline]
    pub fn push(&mut self, value: Value) {
        self.stack.get_mut(self.depth).unwrap().push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> ValueResult {
        self.stack
            .get_mut(self.depth)
            .unwrap()
            .pop()
            .ok_or(RuntimeError::EmptyStack)
    }

    #[inline]
    pub fn parent_pop(&mut self) -> ValueResult {
        self.stack
            .get_mut(self.depth - 1)
            .unwrap()
            .pop()
            .ok_or(RuntimeError::EmptyStack)
    }

    #[inline]
    pub fn dump(&self) -> (HashMap<String, Value>, Vec<Value>) {
        (self.variables[0].clone(), self.stack[0].clone())
    }
}
