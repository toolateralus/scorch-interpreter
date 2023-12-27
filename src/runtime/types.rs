use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    frontend::ast::{Node, Visitor},
    runtime::interpreter::Interpreter,
};

use super::typechecker::{Type, TypeChecker};

#[derive(Debug, Clone)]
pub enum Value {
    None(),
    Int(i32),
    Double(f64),
    Bool(bool),
    String(String),
    Return(Option<Box<Value>>),
    Function(Rc<Function>),
    Array(bool, Vec<Variable>),
    List(Rc<RefCell<Vec<Variable>>>),
    Struct {
        name: String,
        context: Box<Context>
    },
}

pub struct Typedef {
    pub name: String,
    pub fields: Vec<(String, Rc<Type>)>,
    pub(crate) type_: Type,
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

// technically this isn't always variable, it's just a declared field.
#[derive(Debug, Clone)]
pub struct Variable {
    pub mutable: bool, // is_mutable?
    pub value: Value, // this could be a function, a struct, a list, an array, a float, a bool, a string, etc.
    pub m_type: Rc<Type>,
}
impl Variable {
    pub fn new(mutable: bool, value: Value, m_type: Rc<Type>) -> Self {
        Variable {
            mutable,
            value,
            m_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub parent: Option<Rc<RefCell<Context>>>,
    pub variables: HashMap<String, Rc<Variable>>,
}

impl Context {
    pub fn find_variable(&self, name: &str) -> Option<Rc<Variable>> {
        match self.variables.get(name) {
            Some(var) => Some(var.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().find_variable(name),
                None => None,
            },
        }
    }
    pub fn insert_variable(&mut self, name: &str, value: Rc<Variable>) -> () {
        let name_str = name.to_string();
        self.variables.insert(name_str, value);
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
pub struct BuiltInFunction {
    pub func: Box<dyn FnMut(&mut Context, &TypeChecker, Vec<Value>) -> Value>,
}
impl BuiltInFunction {
    pub fn new(func: Box<dyn FnMut(&mut Context, &TypeChecker, Vec<Value>) -> Value>) -> Self {
        BuiltInFunction { func }
    }
    pub fn call(&mut self, context : &mut Context, type_checker : &TypeChecker, args: Vec<Value>) -> Value {
        (self.func)(context, type_checker, args)
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
pub trait ContextHelpers {
    fn add_range(&self, _args: &HashMap<String, Value>) -> ();
}
impl Context {
    pub fn new() -> Context {
        Context {
            parent: Option::None,
            variables: HashMap::new(),
        }
    }
}
