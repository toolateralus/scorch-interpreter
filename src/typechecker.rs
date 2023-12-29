use crate::types::Value;
use std::{collections::HashMap, rc::Rc};

use super::types::{Instance, Struct};

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
                    String::from("none"),
                    Rc::new(Type {
                        name: String::from("none"),
                        validator: Box::new(|v| match v {
                            Value::None() => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("int"),
                    Rc::new(Type {
                        name: String::from("int"),
                        validator: Box::new(|v| match v {
                            Value::Int(..) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("double"),
                    Rc::new(Type {
                        name: String::from("double"),
                        validator: Box::new(|v| match v {
                            Value::Double(_) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("dynamic"),
                    Rc::new(Type {
                        name: String::from("dynamic"),
                        validator: Box::new(|v| match v {
                            _ => true, // :D
                        }),
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("string"),
                    Rc::new(Type {
                        name: String::from("string"),
                        validator: Box::new(|v| match v {
                            Value::String(_) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("bool"),
                    Rc::new(Type {
                        name: String::from("bool"),
                        validator: Box::new(|v| match v {
                            Value::Bool(_) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    }),
                ),
                (
                    String::from("array"),
                    Rc::new(Type {
                        name: String::from("array"),
                        validator: Box::new(|v| match v {
                            Value::Array(..) => true,
                            _ => false,
                        }),
                        attribute: Attr::Array,
                    }),
                ),
                (
                    String::from("fn"),
                    Rc::new(Type {
                        name: String::from("fn"),
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
        match &val{
            Value::Struct{typename, ..} => {
                let struct_decl = self.structs.get(typename)?;
                return Some(Rc::clone(&struct_decl.type_));
            },
            Value::Lambda(func) => {
                let function = func.as_function();
                let sig = super::standard_functions::get_function_signature(&function);
                return Some(Rc::new(Type {
                    name: sig,
                    validator: Box::new(|v| 
                        match v {
                            Value::Lambda(func) => {
                                let function = func.as_function();
                                let sig = super::standard_functions::get_function_signature(&function);
                                sig == sig
                            },
                            _ => {
                                todo!()
                            }
                        }
                    ),
                    attribute: Attr::Function,
                }));
            },
            _ => {
                self.get(get_typename(val))
            }
        }
    }
}

pub fn get_typename<'a>(arg: &'a Value) -> &'a str {
    let arg_type_name = match arg {
        Value::Array(..) => "array",
        Value::None() => "none",
        Value::Int(..) => "int",
        Value::Bool(..) => "bool",
        Value::String(..) => "string",
        Value::Double(..) => "double",
        Value::Return(..) => panic!("cannot get the typename of a return node. if you don't know what this means, something has gone seriously wrong."),
        // todo: Fix the lack of type checking for functions,
        // we need a more centralized way of checking types for structs & functions.
        Value::Lambda(func) => {
            let function = func.as_function(); // todo : fix the lambda system, right now its just creating a new function every time its invoked or searched for.
            let _sig = super::standard_functions::get_function_signature(&function);
            "{sig}"
        },
        Value::Function(func) => {
            let _sig = super::standard_functions::get_function_signature(func);
            "{sig}"
        }
        Value::Struct {
            typename,
            context: _,
        } => &typename,
    };
    arg_type_name
}
