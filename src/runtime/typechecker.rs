use crate::runtime::types::Value;
use std::{collections::HashMap, rc::Rc};

use super::types::Variable;

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub validator: Box<fn(&Value) -> bool>,
}

impl Type {
    pub fn validate(&self, val: &Value) -> bool {
        (self.validator)(val)
    }
}

pub struct TypeChecker {
    types: HashMap<String, Rc<Type>>,
}
impl TypeChecker {
    pub fn new() -> Self {
        Self {
            types: HashMap::from([
                
                (
                    String::from("Int"),
                    Rc::new(Type {
                        name: String::from("Int"),
                        validator: Box::new(|v| match v {
                            Value::Int(..) => true,
                            _ => false,
                        }),
                    }),
                ),
                (
                    String::from("Double"),
                    Rc::new(Type {
                        name: String::from("Double"),
                        validator: Box::new(|v| match v {
                            Value::Double(_) => true,
                            _ => false,
                        }),
                    }),
                ),
                (
                    String::from("Dynamic"),
                    Rc::new(Type {
                        name: String::from("Dynamic"),
                        validator: Box::new(|v| match v {
                            _ => true, // :D
                        }),
                    }),
                ),
                (
                    String::from("String"),
                    Rc::new(Type {
                        name: String::from("String"),
                        validator: Box::new(|v| match v {
                            Value::String(_) => true,
                            _ => false,
                        }),
                    }),
                ),
                (
                    String::from("Bool"),
                    Rc::new(Type {
                        name: String::from("Bool"),
                        validator: Box::new(|v| match v {
                            Value::Bool(_) => true,
                            _ => false,
                        }),
                    }),
                ),
                (
                    String::from("Array"),
                    Rc::new(Type {
                        name: String::from("Array"),
                        validator: Box::new(|v| match v {
                            Value::Array(..) => true,
                            Value::List(..) => true,
                            _ => false,
                        }),
                    }),
                ),
                (
                    String::from("Fn"),
                    Rc::new(Type {
                        name: String::from("Fn"),
                        validator: Box::new(|v| match v {
                            Value::Function(..) => true,
                            _ => false,
                        }),
                    }),
                ),
            ]),
        }
    }
}

impl TypeChecker {
    pub fn validate(val: &Variable) -> bool {
        val.m_type.validate(&val.value)
    }
    pub fn _set(&mut self, name: &String, type_: Type) -> () {
        self.types.insert(name.clone(), Rc::new(type_));
    }
    pub fn get(&self, name: &str) -> Option<Rc<Type>> {
        match self.types.get(name) {
            Some(t) => Some(Rc::clone(t)),
            None => None,
        }
    }
}
pub fn _get_type_name<'a>(arg: &'a Value) -> &'a str {
    let arg_type_name = match arg {
        Value::Int(..) => "Int",
        Value::Double(_) => "Double",
        Value::Bool(_) => "Bool",
        Value::String(_) => "String",
        Value::None() => "None",
        Value::Array(..) | Value::List(..) => "Array",
        Value::Function(_func) => "Fn",
        Value::Return(_) => todo!(),
        // not yet implemented
        Value::Struct {
            name: _,
            context: _,
        } => todo!(),
    };
    arg_type_name
}
