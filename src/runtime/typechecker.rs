use std::collections::HashMap;
use crate::runtime::types::Value;

use super::types::Variable;

#[derive(Debug, Clone)] 
pub struct Type {
    name: String,
    validator: Box<fn(Value) -> bool>,
}

#[derive(Debug, Clone)]
pub struct TypeChecker {
    types: HashMap<String, Type>,
}
impl TypeChecker {
    pub fn new() -> Self {
        Self {
            types: HashMap::from([
                (String::from("Float"), Type {
                    name: String::from("Float"),
                    validator: Box::new(|v| match v {
                        Value::Float(_) => true,
                        _ => false,
                    }),
                }),
                (String::from("Dynamic"), Type {
                    name: String::from("Dynamic"),
                    validator: Box::new(|v| match v {
                        _ => true, // :D
                    }),
                }),
                (String::from("String"), Type {
                    name: String::from("String"),
                    validator: Box::new(|v| match v {
                        Value::String(_) => true,
                        _ => false,
                    }),
                }),
                (String::from("Bool"), Type {
                    name: String::from("Bool"),
                    validator: Box::new(|v| match v {
                        Value::Bool(_) => true,
                        _ => false,
                    }),
                }),
            ]),
        }
    }
}

impl TypeChecker {
    pub fn validate(val : &Variable, struct_name : Option<&String>) -> bool {
        let typename = &val.typename;
        
        // temporarily, while we have no Dynamic types due to no structs.
        if typename == "Dynamic" {
            return true;
        }
        
        match &val.value {
            Value::Float(_) => typename == "Float",
            Value::Bool(_) => typename == "Bool",
            Value::String(_) => typename == "String",
            Value::Function(_) => typename == "function",
            Value::Array(_) => typename == "array",
            Value::List(_) => typename == "list",
            Value::Struct { name, .. } => *typename == *name,
            Value::None(_) => typename == "none",
            _ => false,
        }
    }
    pub fn set(&mut self, name: &String, type_ : Type) -> () {
        self.types.insert(name.clone(), type_);
    }
    pub fn get(&self, name: &str) -> Option<Type> {
        match self.types.get(name) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }
}
