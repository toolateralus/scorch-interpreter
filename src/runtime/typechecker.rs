use crate::runtime::types::Value;
use std::{collections::HashMap, rc::Rc, cell::RefCell};

use super::types::Variable;

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub validator: Box<fn(Value) -> bool>,
}

#[derive(Debug, Clone)]
pub struct TypeChecker {
    types: HashMap<String, Rc<RefCell<Type>>>,
}
impl TypeChecker {
    pub fn create(name : String, validator : Box<fn(Value) -> bool>) -> Rc<RefCell<Type>> {
        Rc::new(RefCell::new(Type {
            name,
            validator,
        }))
    }
    pub fn new() -> Self {
        Self {
            types: HashMap::from([
                (
                    String::from("Int"),
                    Self::create(String::from("Int"), Box::new(|v| match v {
                        Value::Int(..) => true,
                        _ => false,
                    })),
                ),
                (
                    String::from("Double"),
                    Self::create(String::from("Double"), Box::new(|v| match v {
                        Value::Double(_) => true,
                        _ => false,
                    })),
                ),
                (
                    String::from("Dynamic"),
                    Self::create(String::from("Dynamic"), Box::new(|v| match v {
                        _ => true, // :D
                    })),
                ),
                (
                    String::from("String"),
                    Self::create(String::from("String"), Box::new(|v| match v {
                        Value::String(_) => true,
                        _ => false,
                    })),
                ),
                (
                    String::from("Bool"),
                    Self::create(String::from("Bool"), Box::new(|v| match v {
                        Value::Bool(_) => true,
                        _ => false,
                    })),
                ),
                (
                    String::from("Array"),
                    Self::create(String::from("Array"), Box::new(|v| match v {
                        Value::Array(..) => true,
                        Value::List(..) => true,
                        _ => false,
                    })),
                ),
                (
                    String::from("Fn"),
                    Self::create(String::from("Fn"), Box::new(|v| match v {
                        Value::Function(..) => true,
                        _ => false,
                    })),
                ),
            ]),
        }
    }
}

impl TypeChecker {
    pub fn validate(val: &Variable, _struct_name: Option<&String>) -> bool {
        let typename = &val.typename;

        // temporarily, while we have no Dynamic types due to no structs.
        if typename == "Dynamic" {
            return true;
        }

        let t = val.type_.clone();

        // invoke type validation function
        let type_valid = (t.try_borrow().unwrap().validator)(val.value.clone());

        type_valid && typename == get_type_name(&val.value)
    }
    pub fn set(&mut self, name: &String, type_: Type) -> () {
        self.types.insert(name.clone(), Rc::new(RefCell::new(type_)));
    }
    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Type>>> {
        match self.types.get(name) {
            Some(typeref) => Some(typeref.clone()),
            None => None,
        }
    }
}
pub fn get_type_name<'a>(arg: &'a Value) -> &'a str {
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
