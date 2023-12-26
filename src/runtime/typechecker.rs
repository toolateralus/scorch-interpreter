use crate::{runtime::types::Value, frontend::ast::Node};
use std::{collections::HashMap, rc::Rc};

use super::{types::{Variable, Typedef}, interpreter::Interpreter};

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
    pub typedefs: HashMap<String, Box<Typedef>>,
}
impl TypeChecker {
    pub fn new() -> Self {
        Self {
            typedefs: HashMap::new(),
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
    pub fn get(&self, name: &str) -> Option<Rc<Type>> {
        match self.types.get(name) {
            Some(t) => Some(Rc::clone(t)),
            None => None,
        }
    }
    pub fn from_value(&self, val: &Value) -> Option<Rc<Type>> {
        self.get(_get_type_name(val))
    }
}
pub fn _get_type_name<'a>(arg: &'a Value) -> &'a str {
    let arg_type_name = match arg {
        Value::Array( .. ) |
        Value::List(..) => "Array",
        Value::None()    => "None",
        Value::Int( .. )   => "Int",
        Value::Bool( .. )   => "Bool",
        Value::String( .. ) => "String",
        Value::Double( .. ) => "Double",
        Value::Function( .. ) => "Fn",
        Value::Return( .. ) => todo!(),
        Value::Struct { .. } => "Struct",
    };
    arg_type_name
}
