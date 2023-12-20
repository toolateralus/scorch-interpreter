use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{frontend::ast::Node, runtime::interpreter::Interpreter};

#[derive(Debug, Clone)]
pub enum ValueType {
    Float(f64),
    Bool(bool),
    String(String),
    None(()),
}
#[derive(Debug, Clone)]
pub struct Context {
    pub parent: Option<Rc<RefCell<Context>>>,
    pub children: Vec<Rc<RefCell<Context>>>,
    // todo: add return values
    pub functions: HashMap<String, Rc<Function>>,
    // todo: implement a Variable struct that can store more data about the var/const etc.
    pub variables: HashMap<String, Rc<ValueType>>,
}

impl Context {
    pub fn find_variable(&self, name: &str) -> Option<Rc<ValueType>> {
        match self.variables.get(name) {
            Some(var) => Some(var.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().find_variable(name),
                None => None,
            },
        }
    }
    pub fn find_function(&self, name: &str) -> Option<Rc<Function>> {
        match self.functions.get(name) {
            Some(var) => Some(var.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().find_function(name),
                None => None,
            },
        }
    }
    pub fn insert_variable(&mut self, name: &str, value: Rc<ValueType>) -> () {
        let name_str = name.to_string();
        self.variables.insert(name_str, value);
    }
    pub fn insert_function(&mut self, name: &str, value: Rc<Function>) -> () {
        if self.variables.contains_key(name) {
            panic!("Redefinition : Function {} already exists", name);
        }
        let name_str = name.to_string();
        self.functions.insert(name_str, value);
    }
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
    func: Box<dyn FnMut(Vec<ValueType>) -> ValueType>,
}
impl BuiltInFunction {
    pub fn new(func: Box<dyn FnMut(Vec<ValueType>) -> ValueType>) -> Self {
        BuiltInFunction { func }
    }
    pub fn call(&mut self, args: Vec<ValueType>) -> ValueType {
        (self.func)(args)
    }
}
pub trait Invokable {
    fn extract_args(
        interpeter: &mut Interpreter,
        arguments: &Option<Vec<Node>>,
        ctx: &Context,
    ) -> Vec<ValueType>;
}
impl Invokable for Function {
    fn extract_args(
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
