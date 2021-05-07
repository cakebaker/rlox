use std::time::SystemTime;

use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::value::Value;

#[derive(Clone)]
pub struct Clock {}

impl Clock {
    pub const fn new() -> Self {
        Self {}
    }
}

impl LoxCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    // Returns the seconds since 1970-01-01
    fn call(&self, _: &Interpreter, _: Vec<Value>) -> Value {
        let since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");

        Value::Number(since_epoch.as_secs_f64())
    }
}
