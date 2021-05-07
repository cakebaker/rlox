use std::fmt;

use crate::interpreter::Interpreter;
use crate::value::Value;

pub trait LoxCallable: CallableClone {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Value;
}

// workaround based on https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928
pub trait CallableClone {
    fn clone_box(&self) -> Box<dyn LoxCallable>;
}

impl<T> CallableClone for T
where
    T: 'static + LoxCallable + Clone,
{
    fn clone_box(&self) -> Box<dyn LoxCallable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn LoxCallable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl fmt::Debug for Box<dyn LoxCallable> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<LoxCallable>")
    }
}

impl PartialEq for Box<dyn LoxCallable> {
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }
}

// workaround for bug https://github.com/rust-lang/rust/issues/31740
impl PartialEq<&Self> for Box<dyn LoxCallable> {
    fn eq(&self, other: &&Self) -> bool {
        *self == *other
    }
}
