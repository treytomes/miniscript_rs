// The environment stores a map of variable name to string/f64 value for global variables.
// The environment is used to resolve variable references in the AST.

use std::collections::HashMap;
use crate::EvalResult;

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Box<Option<Environment>>,
    pub variables: HashMap<String, EvalResult>,
    // pub functions: HashMap<String, Vec<String>>,
}
impl Environment {
    pub fn new_root() -> Self {
        Self {
            enclosing: Box::new(None),
            variables: HashMap::new(),
            // functions: HashMap::new(),
        }
    }

    pub fn new_child(enclosing: Environment) -> Self {
        Self {
            enclosing: Box::new(Some(enclosing)),
            variables: HashMap::new(),
            // functions: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, name: &str, value: &EvalResult) {
        self.variables.insert(name.to_string(), value.clone());
    }
    
    pub fn get(&self, name: &str) -> Result<&EvalResult, String> {
        match self.variables.get(name) {
            Some(value) => Ok(value),
            None => {
                match self.enclosing.as_ref() {
                    Some(ref enclosing) => enclosing.get(name),
                    None => Err(format!("Undefined Identifier: '{}' is unknown in this context.", name)),
                }
            },
        }
    }
    
    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
    
    pub fn remove(&mut self, name: &str) {
        self.variables.remove(name);
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }
}