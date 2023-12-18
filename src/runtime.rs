use std::{collections::HashMap, f64::NAN};
use crate::ast::{Visitor, Node};
#[derive(Debug, Clone)]
pub enum ValueType {
    Float(f64),
    Bool(bool),
    String(String),
    None(()),
}
#[derive(Debug)]
pub struct Context {
    pub parent: Option<Box<Context>>,
    pub children: Vec<Box<Context>>,
    pub variables: HashMap<String, Box<ValueType>>,
}
impl Context {
    pub fn new() -> Context {
        Context {
            parent: Option::None,
            children: Vec::new(),
            variables: HashMap::new(),
        }
    }
}
#[derive(Debug)]
pub struct Interpreter {
    pub context: Context,
}
impl Visitor<ValueType> for Interpreter {
    fn visit_block(&mut self, node: &Node) -> ValueType {
        if let Node::Block(statements) = node {
            for statement in statements {
                statement.accept(self);
            }
        } else {
            panic!("Expected Block node");
        }
        return ValueType::None(());
    }
    fn visit_declaration(&mut self, node: &Node) -> ValueType {
        if let Node::DeclStmt {
            target_type,
            id,
            expression,
        } = node
        {
            let mut value = ValueType::None(());
            
            match target_type.as_str() {
                "num"   |
                "string" => {
                    value = self.visit_expression(expression);
                }
                _ => { 
                    dbg!(node);
                    panic!("Unsupported type");
                }
            }

            let str_id : String = id.clone();

            // redefinition
            if self.context.variables.contains_key(&str_id) {
                dbg!(node);
                panic!("redefinition of variable");
            }
            // first time declaration
            else {
                self.context.variables.insert(str_id, Box::new(value));
            }
            


        } else {
            panic!("Expected Declaration node");
        }        
        return ValueType::None(());
    }
    fn visit_identifier(&mut self, _node: &Node) -> ValueType {
        return ValueType::None(());
    }
    fn visit_number(&mut self, _node: &Node) -> ValueType {
        return ValueType::None(());
    }
    fn visit_term(&mut self, _node: &Node) -> ValueType {
        return ValueType::None(());
    }
    fn visit_factor(&mut self, node: &Node) -> ValueType {
        match node {
            Node::Number(value) => return ValueType::Float(*value),
            Node::Identifier(_id) => {
                match self.context.variables.get(_id) {
                    Some(value) => *value.clone(), // todo : fix copy on value type references here.
                    None => {
                        dbg!(node);
                        panic!("Variable not found");
                    }
                }
            }
            Node::Expression(root) => {
                root.accept(self)
            }
            _ => {
                dbg!(node);
                panic!("Expected Number or Identifier node");
            }
        }
    }
    fn visit_assignment(&mut self, node: &Node) -> ValueType {
        match node {
            Node::AssignStmnt { id, expression } => {
                let mut val = ValueType::None(());
                val = self.visit_expression(expression);
                let str_id : String = match id.as_ref() {
                    Node::Identifier(id) => id.clone(),
                    _ => {
                        dbg!(node);
                        panic!("Expected Identifier node");
                    }
                };
                match self.context.variables.get_mut(&str_id) {
                    Some(value) => {
                        *value = Box::new(val.clone());
                    }
                    None => {
                        dbg!(node);
                        panic!("Variable not found");
                    }
                }
                return ValueType::None(());
            },
            _ => {
                dbg!(node);
                panic!("Expected Assignment node");
            }
        }
    }
    fn visit_binary_op(&mut self, node: &Node) -> ValueType {

        match node {
            Node::AddOp(_, _) |
            Node::SubOp(_, _) |
            Node::MulOp(_, _) |
            Node::DivOp(_, _) => {
                self.bin_op_float(node)
            }
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }

    }
    fn visit_string(&mut self, node: &Node) -> ValueType {
        if let Node::String(_value) = node {
            
        } else {
            panic!("Expected String node");
        }
        return ValueType::None(());
    }
    fn visit_expression(&mut self, node: &Node) -> ValueType {
        if let Node::Expression(root) = node {
            return root.accept(self);
        } else {
            panic!("Expected Expression node");
        }
    }
}

impl Interpreter {
    fn bin_op_float(&mut self, node: &Node) -> ValueType {
        let mut result: f64 = NAN;
        let (lhs, rhs): (&Box<Node>, &Box<Node>) = match node {
            Node::AddOp(lhs, rhs) |
            Node::SubOp(lhs, rhs) |
            Node::MulOp(lhs, rhs) |
            Node::DivOp(lhs, rhs) => (lhs, rhs),
            Node::Number(value) => return ValueType::Float(*value),
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            } 
        };
        let f_lhs = match lhs.accept(self) {
            ValueType::Float(value) => value,
            _ =>  { 
                dbg!(node);
                panic!("Expected float or int");
            }
        };
        let f_rhs = match rhs.accept(self) {
            ValueType::Float(value) => value,
            _ => {
                dbg!(node);
                panic!("Expected float or int")
            }
        };
        match node {
            Node::AddOp(_, _) => result = f_lhs + f_rhs,
            Node::SubOp(_, _) => result = f_lhs - f_rhs,
            Node::MulOp(_, _) => result = f_lhs * f_rhs,
            Node::DivOp(_, _) => result = f_lhs / f_rhs,
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }
        ValueType::Float(result)
    }

    fn bin_op_string(&mut self, node: &Node) -> ValueType {
        let mut result: String = String::from("");
        let (lhs, rhs): (&Box<Node>, &Box<Node>) = match node {
            Node::AddOp(lhs, rhs) |
            Node::SubOp(lhs, rhs) |
            Node::MulOp(lhs, rhs) |
            Node::DivOp(lhs, rhs) => (lhs, rhs),
            Node::String(value) => return ValueType::String(value.to_string()),
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        };
        let f_lhs = match lhs.accept(self) {
            ValueType::String(value) => value,
            _ => panic!("Expected float or int"),
        };
        let f_rhs = match rhs.accept(self) {
            ValueType::String(value) => value,
            _ => panic!("Expected float or int"),
        };
        match node {
            Node::AddOp(_, _) => result = f_lhs + &f_rhs,
            _ => panic!("Expected binary operation node"),
        }
        ValueType::String(result)
    }
}