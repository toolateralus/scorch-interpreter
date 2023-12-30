use crate::types::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    pub types: HashMap<String, Rc<RefCell<Type>>>,
    pub structs: HashMap<String, Box<Struct>>,
}
impl TypeChecker {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
            types: HashMap::from([
                (
                    String::from(NONE_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(NONE_TNAME),
                        validator: Box::new(|v| match v {
                            Value::None() => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    })),
                ),
                (
                    String::from(INT_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(INT_TNAME),
                        validator: Box::new(|v| match v {
                            Value::Int(..) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    })),
                ),
                (
                    String::from(DOUBLE_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(DOUBLE_TNAME),
                        validator: Box::new(|v| match v {
                            Value::Double(_) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    })),
                ),
                (
                    String::from(DYNAMIC_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(DYNAMIC_TNAME),
                        validator: Box::new(|v| match v {
                            _ => true, // :D
                        }),
                        attribute: Attr::Value,
                    })),
                ),
                (
                    String::from(STRING_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(STRING_TNAME),
                        validator: Box::new(|v| match v {
                            Value::String(_) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    })),
                ),
                (
                    String::from(BOOL_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(BOOL_TNAME),
                        validator: Box::new(|v| match v {
                            Value::Bool(_) => true,
                            _ => false,
                        }),
                        attribute: Attr::Value,
                    })),
                ),
                (
                    String::from(ARRAY_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(ARRAY_TNAME),
                        validator: Box::new(|v| match v {
                            Value::Array(..) => true,
                            _ => false,
                        }),
                        attribute: Attr::Array,
                    })),
                ),
                (
                    String::from(FN_TNAME),
                    Rc::new(RefCell::new(Type {
                        name: String::from(FN_TNAME),
                        validator: Box::new(|v| match v {
                            Value::Function(..) => true,
                            _ => false,
                        }),
                        attribute: Attr::Function,
                    })),
                ),
            ]),
        }
    }
}

impl TypeChecker {
    pub fn validate(val: &Instance) -> bool {
        val.m_type.borrow().validate(&val.value)
    }
    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Type>>> {
        match self.structs.get(name) {
            Some(t) => Some(Rc::clone(&t.type_)),
            None => match self.types.get(name) {
                Some(t) => Some(Rc::clone(t)),
                None => None,
            },
        }
    }
    pub fn from_value(&self, val: &Value) -> Option<Rc<RefCell<Type>>> {
        match &val {
            Value::Struct { typename, .. } => {
                let struct_decl = self.structs.get(typename)?;
                return Some(Rc::clone(&struct_decl.type_));
            }
            _ => {
                let typename = get_typename(val);
                let result = self.get(&typename);
                assert!(result.is_some(), "type not found, {}", typename);
                Some(result.unwrap())
            }
        }
    }
}

// import constants.
use scorch_parser::ast::*;

pub fn get_typename(arg: &Value) -> &str {
    match &arg {
        Value::Array(..) => ARRAY_TNAME,
        Value::None() => NONE_TNAME,
        Value::Int(..) => INT_TNAME,
        Value::Bool(..) => BOOL_TNAME,
        Value::String(..) => STRING_TNAME,
        Value::Double(..) => DOUBLE_TNAME,
        Value::Return(..) => panic!("cannot get the typename of a return node. if you don't know what this means, something has gone seriously wrong."),
        // todo: Fix the lack of type checking for functions,
        // we need a more centralized way of checking types for structs & functions.
        Value::Function(..) => FN_TNAME,
        Value::Struct {
            typename,
            context: _,
        } => typename,
        _ => {
            panic!("cannot find type from value {:?}", arg);
        }
    }
}
