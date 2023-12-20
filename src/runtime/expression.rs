use std::rc::Rc;

use crate::{
    frontend::ast::Node,
    runtime::interpreter::Interpreter,
    runtime::types::{Parameter, ValueType},
};

// loops
impl Interpreter {
    pub fn visit_conditional_repeat_stmnt(&mut self, id: &str, condition: &Option<Box<Node>>, block: &Box<Node>) -> ValueType {
        match self.context.find_variable(&id) {
            Some(_) => {
                
            }
            None => {
                self.context.insert_variable(&id, Rc::new(ValueType::Float(0.0)));
            }
        }
        self.context.insert_variable(&id, Rc::new(ValueType::Float(0.0)));
    
        let mut iter : f64 = 0.0;
        loop {
            let condition_result = match condition.as_ref() {
                Some(expression) => {
                    if let ValueType::Bool(val) = expression.accept(self) {
                        val
                    } else {
                        panic!("Expected boolean condition");
                    }
                }
                None => panic!("Expected condition in conditional repeat statement"),
            };
    
            if condition_result {
                block.accept(self);
            } else {
                return ValueType::None(());
            }
            self.context.variables.remove(id);
            
            iter += 1.0;
            
            // should we floor this here?
            self.context.insert_variable(&id, Rc::new(ValueType::Float(iter.floor()))); 
        }
    }
    pub fn visit_conditionless_repeat_stmnt(&mut self, block: &Box<Node>) -> ValueType {
        loop {
            block.accept(self);
        }
    }
}

// binary operation definitions
impl Interpreter {
    pub fn get_params_list(&mut self, param_nodes: &Vec<Node>) -> Vec<Parameter> {
        let mut params = Vec::new();
        for param in param_nodes {
            if let Node::ParamDeclNode { varname, typename } = param {
                let param_name = match varname.as_ref() {
                    Node::Identifier(id) => id.clone(),
                    _ => {
                        dbg!(varname);
                        panic!("Expected Identifier node");
                    }
                };

                let type_name = match typename.as_ref() {
                    Node::Identifier(id) => id.clone(),
                    _ => {
                        dbg!(typename);
                        panic!("Expected Identifier node");
                    }
                };

                let parameter = Parameter {
                    name: param_name,
                    typename: type_name,
                };

                params.push(parameter);
            }
        }
        params
    }
    pub fn bin_op_float(&mut self, node: &Node, lhs: &f64, rhs: &f64) -> ValueType {
        let result: f64;
        match node {
            Node::AddOp(_, _) => result = lhs + rhs,
            Node::SubOp(_, _) => result = lhs - rhs,
            Node::MulOp(_, _) => result = lhs * rhs,
            Node::DivOp(_, _) => result = lhs / rhs,
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }
        ValueType::Float(result)
    }
    pub fn bin_op_string(&mut self, node: &Node, lhs: &String, rhs: &String) -> ValueType {
        let result: String;
        match node {
            Node::AddOp(_, _) => result = format!("{}{}", lhs, rhs),
            _ => {
                dbg!(node);
                panic!("invalid binary operation on strings");
            }
        }
        ValueType::String(result)
    }
}
