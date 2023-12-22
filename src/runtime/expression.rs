use core::panic;
use std::{collections::HashMap, rc::Rc};

use crate::{
    frontend::ast::Node,
    runtime::interpreter::Interpreter,
    runtime::types::{Parameter, Value},
};

use super::types::{BuiltInFunction, Variable};


fn print_ln(args: Vec<Value>) -> Value {
    for arg in args {
        match arg {
            Value::Float(val) => print!("{}\n", val),
            Value::Bool(val) => print!("{}\n", val),
            Value::String(val) => print!("{}\n", val),
            Value::None() => print!("{:?}", Value::None()),
            Value::Function(_) => {
                let newargs = Vec::from([arg.clone()]);
                return tostr(newargs);
            }
            Value::Array(mutable, elements) => {
                
                let mutable_str = if mutable {
                    "mutable"
                } else {
                    "immutable"
                };
                 
                println!("{} array, length {}", mutable_str, elements.len());
                
                for element in elements.iter() {
                    print_ln(Vec::from([element.value.clone()]));
                }
            }
            Value::List(elements) => {
                for element in elements.try_borrow().unwrap().iter() {
                    print_ln(Vec::from([element.value.clone()]));
                }
            }
            Value::Struct { name: _, context: _ } => todo!(),
            Value::Return(_) => panic!("Cannot print return value"),
            
        }
    }
    Value::None()
}
fn wait(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("sleep expected 1 argument :: ms sleep duration");
    }
    if let Value::Float(val) = args[0] {
        std::thread::sleep(std::time::Duration::from_millis(val as u64));
    } else {
        panic!("sleep expected a <num>");
    }
    Value::None()
}
fn readln(args: Vec<Value>) -> Value {
    if args.len() != 0 {
        panic!("readln expected 0 arguments");
    }
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    Value::String(input.replace("\n", ""))
}

fn tostr(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("tostr expected 1 argument");
    }
    let arg = &args[0];
    let result = match arg {
        Value::Float(val) => val.to_string(),
        Value::Bool(val) => val.to_string(),
        Value::String(val) => val.clone(),
        Value::None() => String::from("None"),
        Value::Function(func) => {
            let params: Vec<String> = func
                .params
                .iter()
                .map(|param| format!("{}: {}", param.name, param.typename))
                .collect();
            let stri = String::from(format!("{}({}) -> {}", func.name, params.join(", "), func.return_type));
            println!("{}", stri);
            stri
        }
        Value::Array(mutable, elements) => {
            let mutable_str = if *mutable {
                "mutable"
            } else {
                "immutable"
            };
            format!("array : {} , length : {}", mutable_str, elements.len())
        }
        _ => {
            panic!("Cannot convert value to string");
        }
    };
    Value::String(result)
}



// todo: move this somewhere more appropriate, and organize the definitions of these
pub fn get_builtin_functions() -> HashMap<String, BuiltInFunction> {
    HashMap::from([
        (
            String::from("println"),
            BuiltInFunction::new(Box::new(print_ln)),
        ),
        (
            String::from("readln"),
            BuiltInFunction::new(Box::new(readln)),
        ),
        (String::from("wait"), BuiltInFunction::new(Box::new(wait))),
    ])
}
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
                let val = Value::Float(0.0);
                    
                let var = Rc::new(Variable::from("Float".to_string(), true, val, self.type_checker.clone()));
                
                self.context.insert_variable(&id, var);
            }
        }

        let mut iter: f64 = 0.0;

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
                    Value::Float(..) | Value::Bool(_) | Value::Function(_) | Value::String(_) => {
                        return result
                    }
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
            
            iter += 1.0;

            let value = Value::Float(iter.floor());
            
            let typename = "Float".to_string();
            
            // todo: fix this terrible variable stuff.
            // should we floor this here?
            let variable = Rc::new(Variable::from(
                typename,
                true,
                value,
                self.type_checker.clone()
            ));
            
            self.context.insert_variable( &id, variable);
        }
    }
    pub fn visit_conditionless_repeat_stmnt(&mut self, block: &Box<Node>) -> Value {
        loop {
            let _result = block.accept(self);
            match _result {
                Value::Return(value) => {
                    if let Some(val) = value {
                        return *val;
                    } else
                    {
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

                let parameter = Parameter {
                    name: param_name,
                    typename: type_name,
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
        Value::Float(result)
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
