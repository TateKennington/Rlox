use crate::lox::interpreter::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    variables: HashMap<String, Value>,
    pub parent: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            variables: HashMap::new(),
            parent: None,
        };
    }

    pub fn set_variable(&mut self, identifier: String, value: Value) {
        self.variables.insert(identifier, value);
    }

    pub fn get_variable(&self, identifier: String) -> &Value {
        self.print();
        if self.variables.contains_key(&identifier) {
            return self.variables.get(&identifier).unwrap();
        }
        if matches!(self.parent, Some(_)) {
            return self.parent.as_ref().unwrap().get_variable(identifier);
        }
        return &Value::Nil;
    }

    pub fn print(&self) {
        println!("{:?}", self.variables);
    }
}
