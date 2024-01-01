use super::{context::Context, typechecker::Type};
use crate::interpreter::Interpreter;
use scorch_parser::ast::{Node, Visitor};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum Value {
    None(),
    Int(i32),
    Bool(bool),
    Double(f64),
    String(String),
    Function(Rc<Function>),
    Return(Option<Box<Value>>),
    Reference(Rc<RefCell<Instance>>),
    Array(bool, Rc<RefCell<Vec<Instance>>>),
    StructInstance { typename: String, context: Box<Context> },
}

impl Value {
    pub fn as_bool(&self) -> Option<&bool> {
        let value = match self {
            Value::Bool(val) => Some(val),
            _ => None,
        };
        value
    }
    pub fn as_float(&self) -> Option<&f64> {
        let value = match self {
            Value::Double(val) => Some(val),
            _ => None,
        };
        value
    }
    pub fn as_string(&self) -> Option<&String> {
        let value = match self {
            Value::String(val) => Some(val),
            _ => None,
        };
        value
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub mutable: bool,
    pub value: Value, 
    pub m_type: Rc<RefCell<Type>>,
}
impl Instance {
    pub fn set_value(&mut self, value: &Value) -> () {
        self.value = value.clone();
    }
    pub fn new(mutable: bool, value: Value, m_type: Rc<RefCell<Type>>) -> Self {
        Instance {
            mutable,
            value,
            m_type,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub m_type: Rc<RefCell<Type>>,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Box<Node>,
    pub return_type: Rc<RefCell<Type>>,
    pub mutable: bool,
}
impl Function {
    // todo: replace manual calls with this in interpreter. can also add more
    // procedure here, like injecting args, context swaps.
    pub fn call(&mut self, i: &mut dyn Visitor<Value>) -> Value {
        return self.body.accept(i);
    }
}
pub trait Invokable {
    fn extract_args(interpeter: &mut Interpreter, arguments: &Option<Vec<Node>>) -> Vec<Value>;
}
impl Invokable for Function {
    fn extract_args(interpeter: &mut Interpreter, arguments: &Option<Vec<Node>>) -> Vec<Value> {
        let mut args = Vec::new();
        let args_col = arguments.as_ref().unwrap();
        for arg in args_col {
            let value = interpeter.eval_deref(arg);
            args.push(value);
        }
        args
    }
}
