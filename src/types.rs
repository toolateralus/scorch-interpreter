use std::collections::HashMap;

use crate::{ast::Node, runtime::Interpreter};

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
    pub functions : HashMap<String, Box<Function>>,
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
pub trait Invokable {
    fn create_args(interpeter: &mut Interpreter, arguments : &Option<Vec<Node>>, ctx: &Context) -> Vec<ValueType>;
}
impl Invokable for Function {
    fn create_args(interpeter: &mut Interpreter, arguments : &Option<Vec<Node>>, ctx: &Context) -> Vec<ValueType> {
        let mut args = Vec::new();
        let args_col = arguments.as_ref().unwrap();
        for arg in args_col {
            let value = arg.accept(interpeter);
            args.push(value);
        }
        args
    }
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

    fn add_range(&self, args: &HashMap<String, ValueType>) -> () {
        // todo: add function arguments to the context.
    }
}
