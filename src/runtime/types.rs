use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{frontend::ast::{Node, Visitor}, runtime::interpreter::Interpreter};

use super::typechecker::{Type, TypeChecker};

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Bool(bool),
    String(String),
    Return(Option<Box<Value>>),
    Function(Rc<Function>),
    
    Array(bool, Vec<Variable>),
    List(Rc<RefCell<Vec<Variable>>>),
    
    Struct {
        name: String,
        context : Rc<RefCell<Context>>,
    },
    None(()),
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
            Value::Float(val) => Some(val),
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
    pub typename: String, // the string type name.
    pub mutable: bool, // is_mutable?
    pub value: Value, // this could be a function, a struct, a list, an array, a float, a bool, a string, etc.
    pub type_ : Rc<RefCell<Type>>,
}
impl Variable {
    pub fn from(tname : String, mutable : bool, value: Value, checker : TypeChecker) -> Self {
        let t = checker.get(tname.as_str());
        
        if t.is_none() {
            panic!("Type {} does not exist", tname);
        }
        
        Variable {
            typename : tname,
            mutable,
            value,
            type_ : Rc::new(RefCell::new(t.unwrap())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub parent: Option<Rc<RefCell<Context>>>,
    pub children: Vec<Rc<RefCell<Context>>>,
    // todo: add return values
    pub functions: HashMap<String, Rc<Function>>,
    // todo: implement a Variable struct that can store more data about the var/const etc.
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
    pub fn find_function(&self, name: &str) -> Option<Rc<Function>> {
        match self.functions.get(name) {
            Some(var) => Some(var.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().find_function(name),
                None => None,
            },
        }
    }
    pub fn insert_variable(&mut self, name: &str, value: Rc<Variable>) -> () {
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
    pub mutable : bool,
}

impl Function {
    // todo: replace manual calls with this in interpreter. can also add more 
    // procedure here, like injecting args, context swaps.
    pub fn call(&mut self, i : &mut dyn Visitor<Value>) -> Value {
        return self.body.accept(i);
    }
}
pub struct BuiltInFunction {
    func: Box<dyn FnMut(Vec<Value>) -> Value>,
}
impl BuiltInFunction {
    pub fn new(func: Box<dyn FnMut(Vec<Value>) -> Value>) -> Self {
        BuiltInFunction { func }
    }
    pub fn call(&mut self, args: Vec<Value>) -> Value {
        (self.func)(args)
    }
}
pub trait Invokable {
    fn extract_args(
        interpeter: &mut Interpreter,
        arguments: &Option<Vec<Node>>,
        ctx: &Context,
    ) -> Vec<Value>;
}
impl Invokable for Function {
    fn extract_args(
        interpeter: &mut Interpreter,
        arguments: &Option<Vec<Node>>,
        _ctx: &Context,
    ) -> Vec<Value> {
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
            children: Vec::new(),
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

impl ContextHelpers for Context {
    fn add_range(&self, _args: &HashMap<String, Value>) -> () {
        todo!()
    }
}
