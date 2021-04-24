use crate::interpreter::RuntimeError;
use crate::literal::Literal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Literal>,
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

    // If the variable is present in the environment (or it's parent environments, if any), it's
    // value is updated, and the old value is returned. Otherwise, None is returned.
    pub fn assign(&mut self, name: String, value: Literal) -> Option<Literal> {
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

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Literal, RuntimeError> {
        match self.values.get(&name) {
            Some(literal) => Ok(literal.clone()),
            None => match &self.parent {
                Some(c) => {
                    let env = c.borrow();
                    env.get(name)
                }
                None => Err(RuntimeError {}),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;
    use crate::literal::Literal;

    #[test]
    fn assign_value_to_var() {
        let mut env = Environment::new();
        env.define("key".to_string(), Literal::String("value".to_string()));

        if let Some(previous_value) =
            env.assign("key".to_string(), Literal::String("new value".to_string()))
        {
            assert_eq!(previous_value, Literal::String("value".to_string()));
        } else {
            panic!("Environment::assign() returned unexpected None");
        }

        if let Ok(result) = env.get("key".to_string()) {
            assert_eq!(Literal::String("new value".to_string()), result);
        } else {
            panic!("Environment::get() returned unexpected Err");
        }
    }

    #[test]
    fn assign_value_to_var_from_parent_environment() {
        let mut parent = Environment::new();
        parent.define("key".to_string(), Literal::String("value".to_string()));

        let mut env = Environment::new_with_parent(parent);

        if let Some(previous_value) =
            env.assign("key".to_string(), Literal::String("new value".to_string()))
        {
            assert_eq!(previous_value, Literal::String("value".to_string()));
        } else {
            panic!("Environment::assign() returned unexpected None");
        }

        if let Ok(result) = env.get("key".to_string()) {
            assert_eq!(Literal::String("new value".to_string()), result);
        } else {
            panic!("Environment::get() returned unexpected Err");
        }
    }

    #[test]
    fn assign_value_to_undefined_var() {
        let mut env = Environment::new();
        let previous_value = env.assign("key".to_string(), Literal::String("value".to_string()));

        assert_eq!(previous_value, None);
    }

    #[test]
    fn get_value_of_var() {
        let mut env = Environment::new();
        env.define("key".to_string(), Literal::String("value".to_string()));

        if let Ok(result) = env.get("key".to_string()) {
            assert_eq!(Literal::String("value".to_string()), result);
        } else {
            panic!("Environment::get() returned unexpected Err");
        }
    }

    #[test]
    fn get_value_of_var_from_parent_environment() {
        let mut parent = Environment::new();
        parent.define("key".to_string(), Literal::String("value".to_string()));

        let env = Environment::new_with_parent(parent);

        if let Ok(result) = env.get("key".to_string()) {
            assert_eq!(Literal::String("value".to_string()), result);
        } else {
            panic!("Environment::get() returned unexpected Err");
        }
    }

    #[test]
    fn get_value_of_undefined_var() {
        let env = Environment::new();

        if let Err(_) = env.get("key".to_string()) {
            assert!(true);
        } else {
            panic!("Environment::get() didn't return expected Err");
        }
    }
}
