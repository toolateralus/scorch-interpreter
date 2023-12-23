use core::panic;
use std::rc::Rc;

use crate::{
    frontend::ast::Node,
    runtime::interpreter::Interpreter,
    runtime::types::{Parameter, Value},
};

use super::types::Variable;

// loops
impl Interpreter {
    pub fn visit_conditional_repeat_stmnt(
        &mut self,
        id: &str,
        condition: &Option<Box<Node>>,
        block: &Box<Node>,
    ) -> Value {
        match self.context.find_variable(&id) {
            Some(var) => {
                if var.mutable == false {
                    panic!("Cannot mutate immutable variable {} in a repeat loop", id);
                }
            }
            None => {
                let val = Value::Double(0.0);
                let Some(m_type) = self.type_checker.get("Double") else {
                    panic!("Double isnt a type")
                };
                let var = Variable::new(true, val, m_type);

                self.context.insert_variable(&id, Rc::new(var));
            }
        }

        let mut iter: usize = 0;

        loop {
            let condition_result = match condition.as_ref() {
                Some(expression) => {
                    if let Value::Bool(val) = expression.accept(self) {
                        val
                    } else {
                        panic!("Expected boolean condition");
                    }
                }
                None => panic!("Expected condition in conditional repeat statement"),
            };

            if condition_result {
                let result = block.accept(self);
                match result {
                    Value::Int(..)
                    | Value::Double(..)
                    | Value::Bool(_)
                    | Value::Function(_)
                    | Value::String(_) => return result,
                    Value::Return(value) => {
                        if let Some(val) = value {
                            return *val;
                        } else {
                            return Value::None();
                        }
                    }
                    _ => {}
                }
            } else {
                return Value::None();
            }
            self.context.variables.remove(id);

            iter += 1;

            let value = Value::Int(iter as i32);

            let typename = "Int".to_string();

            let Some(m_type) = self.type_checker.get(typename.as_str()) else {
                panic!("{} isnt a type", typename)
            };
            let variable = Rc::new(Variable::new(true, value, m_type));
            
            self.context.insert_variable(&id, variable);
        }
    }
    pub fn visit_conditionless_repeat_stmnt(&mut self, block: &Box<Node>) -> Value {
        loop {
            let _result = block.accept(self);
            match _result {
                Value::Return(value) => {
                    if let Some(val) = value {
                        return *val;
                    } else {
                        return Value::None();
                    }
                }
                _ => {
                    continue;
                }
            }
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

                let Some(m_type) = self.type_checker.get(type_name.as_str()) else {
                    panic!("{} isnt a type", type_name)
                };

                let parameter = Parameter {
                    name: param_name,
                    m_type,
                };

                params.push(parameter);
            }
        }
        params
    }
    pub fn bin_op_float(&mut self, node: &Node, lhs: &f64, rhs: &f64) -> Value {
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
        Value::Double(result)
    }
    pub fn bin_op_int(&mut self, node: &Node, lhs: &i32, rhs: &i32) -> Value {
        let result: i32;
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
        Value::Int(result)
    }
    pub fn bin_op_string(&mut self, node: &Node, lhs: &String, rhs: &String) -> Value {
        let result: String;
        match node {
            Node::AddOp(_, _) => result = format!("{}{}", lhs, rhs),
            _ => {
                dbg!(node);
                panic!("invalid binary operation on strings");
            }
        }
        Value::String(result)
    }
}
