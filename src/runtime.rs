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
                "dynamic" |
                "num"     |
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
            Node::AddOp(lhs, rhs) |
            Node::SubOp(lhs, rhs) |
            Node::MulOp(lhs, rhs) |
            Node::DivOp(lhs, rhs) => {
                let e_lhs = lhs.accept(self);
                let e_rhs = rhs.accept(self);
                match (e_lhs, e_rhs) {
                    (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                        return self.bin_op_float(node, &lhs_float, &rhs_float);
                    }
                    (ValueType::String(lhs_string), ValueType::String(rhs_string)) => {
                        return self.bin_op_string(node, &lhs_string, &rhs_string);
                    }
                    _ => {
                        dbg!(node);
                        panic!("Expected float or int");
                    }
                }
            }
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }

    }
    fn visit_string(&mut self, node: &Node) -> ValueType {
        if let Node::String(_value) = node {
            return ValueType::String(_value.clone());
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
    fn visit_eof(&mut self, node: &Node) -> ValueType {
        ValueType::None(()) // do nothing.
    }
    fn visit_not_op(&mut self, node: &Node) -> ValueType {
        if let Node::NotOp(operand) = node {
            match operand.accept(self) {
                ValueType::Bool(value) => ValueType::Bool(!value),
                _ => panic!("Expected boolean operand for not operation"),
            }
        } else {
            panic!("Expected NotOp node");
        }
    }
    fn visit_neg_op(&mut self, node: &Node) -> ValueType {
        if let Node::NegOp(operand) = node {
            match operand.accept(self) {
                ValueType::Float(value) => ValueType::Float(-value),
                _ => panic!("Expected numeric operand for negation operation"),
            }
        } else {
            panic!("Expected NegOp node");
        }
    }
}

impl Interpreter {
    fn bin_op_float(&mut self, node : &Node, lhs : &f64, rhs : &f64) -> ValueType {
        let mut result: f64 = NAN;
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

    fn bin_op_string(&mut self, node : &Node, lhs : &String, rhs : &String) -> ValueType {
        let result: String;
        match node {
            Node::AddOp(_, _) => result = format!("{}{}", lhs, rhs),
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }
        ValueType::String(result)
    }
}