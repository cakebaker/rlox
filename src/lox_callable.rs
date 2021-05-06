use crate::interpreter::Interpreter;
use crate::value::Value;

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: Interpreter, arguments: Vec<Value>) -> Value;
}
