use crate::lox::interpreter::Value;
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn set_variable(&mut self, identifier: String, value: Value) {
        self.variables.insert(identifier, value);
    }

    pub fn get_variable(&self, identifier: String) -> Value {
        if self.variables.contains_key(&identifier) {
            return *self.variables.get(&identifier).unwrap();
        }
        if matches!(self.parent, Some(_)) {
            return self.parent.as_ref().unwrap().get_variable(identifier);
        }
        return Value::Nil;
    }
}
