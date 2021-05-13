use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::interpreter::RuntimeError;
use crate::lox_callable::LoxCallable;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::value::Value;

#[derive(Clone)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl LoxFunction {
    pub fn new(name: &Token, params: &[Token], body: &[Stmt]) -> Self {
        Self {
            name: name.clone(),
            params: params.to_owned(),
            body: body.to_owned(),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let mut env = Environment::new_with_parent(interpreter.environment.clone());

        for (i, param) in self.params.iter().enumerate() {
            env.define(param.lexeme.clone(), arguments[i].clone());
        }

        let result = interpreter.execute_block(&self.body, &env);

        match result {
            Err(RuntimeError::Return(value)) => value,
            _ => Value::Nil,
        }
    }
}
