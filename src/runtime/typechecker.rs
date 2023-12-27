use crate::runtime::types::Value;
use std::{collections::HashMap, rc::Rc};

use super::types::{Struct, Variable};

#[derive(Debug, PartialEq)]
pub enum Attr {
    Struct,
    Value,
    Array,
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub validator: Box<fn(&Value) -> bool>,
    pub attribute: Attr,
}

impl Type {
    pub fn validate(&self, val: &Value) -> bool {
        (self.validator)(val)
    }
}

pub struct TypeChecker {
    pub types: HashMap<String, Rc<Type>>,
    pub structs: HashMap<String, Box<Struct>>,
}
impl TypeChecker {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
            types: HashMap::from([
                (
                    String::from("Int"),
                    Rc::new(Type {
                        name: String::from("Int"),
                        validator: Box::new(|v| match v {
                            Value::Int(..) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
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
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("Dynamic"),
                    Rc::new(Type {
                        name: String::from("Dynamic"),
                        validator: Box::new(|v| match v {
                            _ => true, // :D
                        }),
                        attribute: Attr::Value,
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
                        attribute: Attr::Value,
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
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("Array"),
                    Rc::new(Type {
                        name: String::from("Array"),
                        validator: Box::new(|v| match v {
                            Value::Array(..) => true,
                            _ => false,
                        }),
                        attribute: Attr::Array,
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
                        attribute: Attr::Value,
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
    pub fn get(&self, name: &str) -> Option<Rc<Type>> {
        match self.structs.get(name) {
            Some(t) => Some(Rc::clone(&t.type_)),
            None => match self.types.get(name) {
                Some(t) => Some(Rc::clone(t)),
                None => None,
            },
        }
    }
    pub fn from_value(&self, val: &Value) -> Option<Rc<Type>> {
        if let Value::Struct { typename: name, .. } = &val {
            let struct_decl = self.structs.get(name)?;
            return Some(Rc::clone(&struct_decl.type_));
        }

        self.get(_get_type_name(val))
    }
}
pub fn _get_type_name<'a>(arg: &'a Value) -> &'a str {
    let arg_type_name = match arg {
        Value::Array(..) => "Array",
        Value::None() => "None",
        Value::Int(..) => "Int",
        Value::Bool(..) => "Bool",
        Value::String(..) => "String",
        Value::Double(..) => "Double",
        Value::Function(..) => "Fn",
        Value::Return(..) => todo!(),
        Value::Struct { .. } => "Struct",
    };
    arg_type_name
}
