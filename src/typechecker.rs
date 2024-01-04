use crate::{types::{Value, Function}, context::Context};

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
    pub rust_method : Option<Box<dyn Fn(&Value, &Value) -> Value>>,
    pub user_fn : Option<Rc<Function>>,
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
                if let Some(op_ovr_mthod) = op_ovr.rust_method.as_ref() {
                    return (op_ovr_mthod)(lhs_value, other);
                } else {
                    Value::None()
                }
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
    pub fn from_value(&mut self, val: &Value) -> Option<Rc<RefCell<Type>>> {
        let type_name = self.get_typename(val);
        let result = self.get(&type_name);
        assert!(result.is_some(), "typechecker::from_value() failed to get type from value {:#?}", val);
        Some(result.unwrap())
    }
    pub fn get_tuple_typename(&mut self, values: &Vec<Value>) -> String {
        let mut typename = String::from("(");
        for value in values {
            if values.len() != 1 {
                typename.push_str(",");
            }
            
            let value_tname = &self.get_typename(value);
            
            if self.types.get(value_tname).is_none() {
                panic!("invalid or undefined type expression in value tuple : {:?} typename {:?}", value, typename);
            }
            
            typename.push_str(&value_tname);
            
            
        }
        typename.push_str(")");
        
        if self.get(&typename).is_none() {
            self.types.insert(typename.to_string(), Rc::new(RefCell::new(Type {
                name: typename.clone(),
                validator: Box::new(|v| match v {
                    Value::Tuple(_values) => {
                        true // todo #25 : add better typechecking to tuples & arrays.
                    },
                    _ => false,
                }),
                attribute: Attr::Value,
                context: Box::new(Context { parent: None, variables: HashMap::new() }),
                operators: Vec::new(),
            })));
        }
        typename
    }
    pub fn get_typename(&mut self, arg: &Value) -> String {
        match arg {
            Value::Array(..) => String::from(ARRAY_TNAME),
            Value::None() => String::from(NONE_TNAME),
            Value::Int(..) => String::from(INT_TNAME),
            Value::Bool(..) => String::from(BOOL_TNAME),
            Value::String(..) => String::from(STRING_TNAME),
            Value::Double(..) => String::from(DOUBLE_TNAME),
            Value::Return(..) => panic!("Cannot get the typename of a return node. If you don't know what this means, something has gone seriously wrong."),
            Value::Function(..) => String::from(FN_TNAME),
            Value::StructInstance { typename, context: _ } => typename.clone(),
            Value::Reference(refcell) =>{
                let refcell = refcell.borrow();
                let instance = &refcell.value;
                let typename = self.get_typename(instance);
                typename
            },
            Value::Tuple(values) => {
                let typename = self.get_tuple_typename(values);
                typename
            }
            Value::KeyTypeTuple(..) => panic!("this type cannot be instantiated."),
            Value::KeyTypePair(..) => panic!("this type cannot be instantiated."),
        }
    }
}
use scorch_parser::{ast::*, lexer::TokenKind};


