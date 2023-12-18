use std::{collections::HashMap, f64::NAN};
use crate::ast::{Visitor, Node};
#[derive(Debug)]
pub enum ValueType {
    Float(f64),
    Int(i64),
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
                "float" => {
                    value = self.bin_op_float(node);
                },
                "string" => {
                    value = self.bin_op_string(node);
                },
                _ => {
                    panic!("Unsupported type");
                }
            }
            self.context.variables.insert(id.to_string(), Box::new(value));

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
        if let Node::Number(value) = node {
            return ValueType::Float(*value);           
        } else if let Node::Identifier(_id) = node {
            // todo: dereference identifiers
            // id == string id;

        } else if let Node::Expression(root) = node {
            root.accept(self);
        } else {
            dbg!(node);
            panic!("Expected Number or Identifier node");
        }
        return ValueType::None(());
    }
    fn visit_assignment(&mut self, _node: &Node) -> ValueType {
        return ValueType::None(());
    }
    fn visit_binary_op(&mut self, node: &Node) -> ValueType {
        self.bin_op_float(node)
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
            _ => panic!("Expected binary operation node"),
        };
        let f_lhs = match lhs.accept(self) {
            ValueType::Float(value) => value,
            ValueType::Int(value) => value as f64,
            _ => panic!("Expected float or int"),
        };
        let f_rhs = match rhs.accept(self) {
            ValueType::Float(value) => value,
            ValueType::Int(value) => value as f64,
            _ => panic!("Expected float or int"),
        };
        match node {
            Node::AddOp(_, _) => result = f_lhs + f_rhs,
            Node::SubOp(_, _) => result = f_lhs - f_rhs,
            Node::MulOp(_, _) => result = f_lhs * f_rhs,
            Node::DivOp(_, _) => result = f_lhs / f_rhs,
            _ => panic!("Expected binary operation node"),
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