use crate::interpreter::RuntimeError;
use crate::literal::Literal;
use crate::token::Token;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: String, value: Literal) {
        if let Some(x) = self.values.get_mut(&name) {
            *x = value;
        }
        // TODO throw Error if key is not in map
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => Err(RuntimeError {}),
        }
    }
}
