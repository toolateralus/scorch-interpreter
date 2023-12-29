use super::{context::Context, typechecker::Type};
use crate::interpreter::Interpreter;
use scorch_parser::{
    ast::{Node, Visitor},
    parser::generate_random_function_name,
};
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
    Array(bool, Rc<RefCell<Vec<Instance>>>),
    Struct {
        typename: String,
        context: Box<Context>,
    },
    Lambda(Rc<Lambda>),
}
#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: Vec<Parameter>,
    pub block: Box<Node>,
    pub return_type: Rc<Type>,
}
impl Lambda {
    pub(crate) fn as_function(&self) -> Rc<Function> {
        Rc::new(Function {
            name: generate_random_function_name(),
            params: self.params.clone(),
            body: self.block.clone(),
            return_type: Rc::clone(&self.return_type),
            mutable: false,
        })
    }
}
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, Rc<Type>)>,
    pub type_: Rc<Type>,
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
// technically this isn't always variable, it's just a declared field or value in an array.
#[derive(Debug, Clone)]
pub struct Instance {
    pub mutable: bool, // is_mutable?
    pub value: Value, // this could be a function, a struct, a list, an array, a float, a bool, a string, etc.
    pub m_type: Rc<Type>,
}
impl Instance {
    pub fn set_value(&mut self, value: &Value) -> () {
        self.value = value.clone(); // todo : stop cloning every value on assignment? maybe use Rc?
    }
    pub fn new(mutable: bool, value: Value, m_type: Rc<Type>) -> Self {
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
    pub m_type: Rc<Type>,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Box<Node>,
    pub return_type: Rc<Type>,
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
            let value = arg.accept(interpeter);
            args.push(value);
        }
        args
    }
}
