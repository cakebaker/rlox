use crate::error::RuntimeError;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: Self) -> Self {
        Self {
            parent: Some(Rc::new(RefCell::new(parent))),
            values: HashMap::new(),
        }
    }

    // If the variable is present in the environment (or its parent environments, if any), its
    // value is updated, and the old value is returned. Otherwise, None is returned.
    pub fn assign(&mut self, name: String, value: Value) -> Option<Value> {
        match self.values.get_mut(&name) {
            Some(x) => {
                let old_value = x.clone();
                *x = value;

                Some(old_value)
            }
            None => match &self.parent {
                Some(c) => {
                    let mut env = c.borrow_mut();
                    env.assign(name, value)
                }
                None => None,
            },
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Value, RuntimeError> {
        match self.values.get(&name) {
            Some(literal) => Ok(literal.clone()),
            None => match &self.parent {
                Some(c) => {
                    let env = c.borrow();
                    env.get(name)
                }
                None => Err(RuntimeError::UndefinedVariable(name)),
            },
        }
    }

    // Takes the parent, leaving None in its place.
    pub fn take_parent(&mut self) -> Option<Self> {
        if self.parent.is_none() {
            None
        } else {
            let parent = Some(self.parent.as_ref().unwrap().take());
            self.parent = None;
            parent
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_value_to_var() {
        let mut env = Environment::new();
        env.define("key".to_string(), Value::String("value".to_string()));

        let previous_value = env
            .assign("key".to_string(), Value::String("new value".to_string()))
            .unwrap();
        assert_eq!(previous_value, Value::String("value".to_string()));

        let result = env.get("key".to_string()).unwrap();
        assert_eq!(Value::String("new value".to_string()), result);
    }

    #[test]
    fn assign_value_to_var_from_parent_environment() {
        let mut parent = Environment::new();
        parent.define("key".to_string(), Value::String("value".to_string()));

        let mut env = Environment::new_with_parent(parent);

        let previous_value = env
            .assign("key".to_string(), Value::String("new value".to_string()))
            .unwrap();
        assert_eq!(previous_value, Value::String("value".to_string()));

        let result = env.get("key".to_string()).unwrap();
        assert_eq!(Value::String("new value".to_string()), result);
    }

    #[test]
    fn assign_value_to_undefined_var() {
        let mut env = Environment::new();
        let previous_value = env.assign("key".to_string(), Value::String("value".to_string()));

        assert_eq!(previous_value, None);
    }

    #[test]
    fn get_value_of_var() {
        let mut env = Environment::new();
        env.define("key".to_string(), Value::String("value".to_string()));

        let result = env.get("key".to_string()).unwrap();
        assert_eq!(Value::String("value".to_string()), result);
    }

    #[test]
    fn get_value_of_var_from_parent_environment() {
        let mut parent = Environment::new();
        parent.define("key".to_string(), Value::String("value".to_string()));

        let env = Environment::new_with_parent(parent);

        let result = env.get("key".to_string()).unwrap();
        assert_eq!(Value::String("value".to_string()), result);
    }

    #[test]
    fn get_value_of_undefined_var() {
        let env = Environment::new();
        let error = env.get("key".to_string()).unwrap_err();

        match error {
            RuntimeError::UndefinedVariable(var) => assert_eq!("key".to_string(), var),
            _ => assert!(false),
        }
    }

    #[test]
    fn take_parent() {
        let parent = Environment::new();
        let mut env = Environment::new_with_parent(parent.clone());

        let result = env.take_parent().unwrap();
        assert_eq!(parent, result);
        assert!(env.take_parent().is_none());
    }

    #[test]
    fn take_parent_if_there_is_none() {
        let result = Environment::new().take_parent();

        assert!(result.is_none());
    }
}
