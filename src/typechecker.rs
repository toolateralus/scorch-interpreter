use crate::{types::Value, context::Context};

use super::types::Instance;
use std::{fmt::Debug, rc::Rc, cell::RefCell, collections::HashMap};

#[derive(Debug, PartialEq)]
pub enum Attr {
    Struct,
    Value,
    Array,
    Function,
}

pub struct OperatorOverload {
    pub rhs_t: String,
    pub op : TokenKind,
    pub method : Box<dyn Fn(&Value, &Value) -> Value + 'static>,
}


pub struct Type {
    pub name: String,
    pub validator: Box<fn(&Value) -> bool>,
    pub attribute: Attr,
    pub operators: Vec<OperatorOverload>,
    pub context : Box<Context>
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type {{\n  name: {}, \n  attribute: {:?} \n  # of operator overloads: {}\n  # of fields on type : {}\n}}",
         self.name, self.attribute, self.operators.len(), self.context.variables.len())
    }
}

impl Type {
    pub fn validate(&self, val: &Value) -> bool {
        (self.validator)(val)
    }
    pub fn perform_bin_op<'a>(&'a self, op: &'a TokenKind, rhs_t: &Rc<RefCell<Type>>, lhs_value : &'a Value, other : &'a Value) -> Value {
        let other_tname = rhs_t.borrow().name.clone();
        
        let op_ovr = self.operators.iter().find(|op_ovr| {
            op_ovr.op == *op && op_ovr.rhs_t == other_tname
        });
        
        match &op_ovr {
            Some(op_ovr) => {
                let result = (op_ovr.method)(lhs_value, other);
                return result.clone();
            },
            None => {
                panic!("no operator overload found operator {:?} for type {} and type {}",op, self.name, other_tname);
            }
        }
    }
}

pub struct TypeChecker {
    pub types: HashMap<String, Rc<RefCell<Type>>>,
}
impl TypeChecker {
    pub fn new() -> Self {
        Self {
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
                        context: Box::new(Context { parent: None, variables: HashMap::new() }),
                        operators: Vec::new(),
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
        match self.types.get(name) {
            Some(t) => Some(Rc::clone(t)),
            None => None,
        }
    }
    pub fn from_value(&self, val: &Value) -> Option<Rc<RefCell<Type>>> {
        let typename = get_typename(val);
        let result = self.get(&typename);
        assert!(result.is_some(), "type not found, {}", typename);
        Some(result.unwrap())
    }
}

// import constants.
use scorch_parser::{ast::*, lexer::TokenKind};

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
        Value::StructInstance {
            typename,
            context: _,
        } => typename,
        _ => {
            panic!("cannot find type from value {:?}", arg);
        }
    }
}
