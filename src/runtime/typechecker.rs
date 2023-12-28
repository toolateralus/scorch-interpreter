use crate::runtime::types::Value;
use std::{collections::HashMap, rc::Rc};

use super::types::{Struct, Instance};

#[derive(Debug, PartialEq)]
pub enum Attr {
    Struct,
    Value,
    Array,
    Function,
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
                    String::from("None"),
                    Rc::new(Type {
                        name: String::from("None"),
                        validator: Box::new(|v| match v {
                            Value::None() => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    }),
                ),
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
                        attribute: Attr::Function,
                    }),
                ),
            ]),
        }
    }
}

impl TypeChecker {
    pub fn validate(val: &Instance) -> bool {
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
        
        self.get(get_typename(val))
    }
}

pub fn get_typename<'a>(arg: &'a Value) -> &'a str {
    let arg_type_name = match arg {
        Value::Array(..) => "Array",
        Value::None() => "None",
        Value::Int(..) => "Int",
        Value::Bool(..) => "Bool",
        Value::String(..) => "String",
        Value::Double(..) => "Double",
        Value::Return(..) => todo!(),
        Value::Lambda { .. } => todo!(),
        // todo: Fix the lack of type checking for functions,
        // we need a more centralized way of checking types for structs & functions.
        Value::Function(func) => {
            let sig = super::std_builtins::get_function_signature(func);
            "{sig}"
        }
        Value::Struct { typename, context : _ } => &typename,
    };
    arg_type_name
}
