use std::collections::HashMap;

use crate::{ast::Node, runtime::interpreter::Interpreter};

#[derive(Debug, Clone)]
pub enum ValueType {
    Float(f64),
    Bool(bool),
    String(String),
    None(()),
}
#[derive(Debug, Clone)]
pub struct Context {
    pub parent: Option<Box<Context>>,
    pub children: Vec<Box<Context>>,
    pub functions: HashMap<String, Box<Function>>,
    pub variables: HashMap<String, Box<ValueType>>,
}
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub typename: String,
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Box<Node>,
    pub return_type: String,
}


pub struct BuiltInFunction {
    id: String,
    func: Box<dyn FnMut(Vec<ValueType>) -> ValueType>,
}

impl BuiltInFunction {
    pub fn new(id: String, func: Box<dyn FnMut(Vec<ValueType>) -> ValueType>) -> Self {
        BuiltInFunction { id, func }
    }
    
    pub fn call(&mut self, args: Vec<ValueType>) -> ValueType {
        (self.func)(args)
    }
}

pub trait Invokable {
    fn create_args(
        interpeter: &mut Interpreter,
        arguments: &Option<Vec<Node>>,
        ctx: &Context,
    ) -> Vec<ValueType>;
}
impl Invokable for Function {
    fn create_args(
        interpeter: &mut Interpreter,
        arguments: &Option<Vec<Node>>,
        _ctx: &Context,
    ) -> Vec<ValueType> {
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
    fn add_range(&self, _args: &HashMap<String, ValueType>) -> ();
}
impl Context {
    pub fn new() -> Context {
        Context {
            parent: Option::None,
            children: Vec::new(),
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

impl ContextHelpers for Context {
    fn add_range(&self, _args: &HashMap<String, ValueType>) -> () {
        todo!()
    }
}
