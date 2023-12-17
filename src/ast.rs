use core::panic;
use std::{f32::consts::E, panic::AssertUnwindSafe};

use crate::tokens::*;

#[derive(Debug)]
pub enum Node {
    // literal & values
    Undefined(),
    Number(f64),
    Identifier(String),

    // binary operations
    AddOp(Box<Node>, Box<Node>),
    SubOp(Box<Node>, Box<Node>),
    MulOp(Box<Node>, Box<Node>),
    DivOp(Box<Node>, Box<Node>),

    // Statements
    AssignStmnt {
        id: Box<Node>,
        expression : Box<Node>,
    },
    DeclStmt {
        target_type: String,
        id : String,
        expression : Box<Node>,
    },
}

impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Node::Undefined() => panic!("visitor reached undefined node."),
            Node::Identifier(_key) => visitor.visit_variable(self),
            Node::Number(_value) => { visitor.visit_number(self) },
            Node::AddOp(_lhs, _rhs) => visitor.visit_term(self),
            Node::SubOp(_lhs, _rhs) => visitor.visit_term(self),
            Node::MulOp(_lhs, _rhs) => visitor.visit_factor(self),
            Node::DivOp(_lhs, _rhs) => visitor.visit_factor(self),
            Node::AssignStmnt {id: _, expression: _} => visitor.visit_assignment(self),
            Node::DeclStmt { target_type, id, expression } => visitor.visit_declaration(self),
        }
    }
}
pub fn parse_program(tokens: &Vec<Token>) -> Node {
    let mut index = 0;
    //parse_expression(tokens, &mut index)
    let program = parse_statement(tokens, &mut index);
    program
}
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let current = tokens.get(*index).unwrap();
    let next = tokens.get(*index + 1).unwrap();

    if current.family != TokenFamily::Identifier {
        panic!("Expected identifier token");
    }
    
    if next.kind == TokenKind::Colon {
        *index += 2;
        // todo: check for valid type / builtins
        let target_type = tokens.get(*index).unwrap().value.clone();
        *index += 1;

        let id = current.value.clone();
        let expression = parse_expression(tokens, index);
        Node::DeclStmt {
            target_type,
            id,
            expression: Box::new(expression),
        }
    }
    else if next.kind == TokenKind::Assignment {
        *index += 2;
        let id = Node::Identifier(current.value.clone());
        let expression = parse_expression(tokens, index);
        Node::AssignStmnt {
            id: Box::new(id),
            expression: Box::new(expression),
        }
    } else {
        panic!("Expected ':' or '=' token after Identifier,\n instead got : \n current : {:?}\n next : {:?}", current, next);
    }

}

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_term(tokens, index);

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Add => {
                *index += 1;
                let right = parse_term(tokens, index);
                left = Node::AddOp(Box::new(left), Box::new(right));
            }
            TokenKind::Subtract => {
                *index += 1;
                let right = parse_term(tokens, index);
                left = Node::SubOp(Box::new(left), Box::new(right));
            },
            TokenKind::CloseParenthesis => break,
            TokenKind::Semicolon => break,
            TokenKind::Comma => break,
            _ => {
                println!("left");
                dbg!(left);
                println!("token");
                dbg!(token);
                panic!("unexpected token");
            },
        }
    }
    left
}
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_addition(tokens, index);
    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Multiply => {
                *index += 1;
                let right = parse_addition(tokens, index);
                left = Node::MulOp(Box::new(left), Box::new(right));
            }
            TokenKind::Divide => {
                *index += 1;
                let right = parse_addition(tokens, index);
                left = Node::DivOp(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    left
}
fn parse_addition(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_factor(tokens, index);

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Add => {
                *index += 1;
                let right = parse_factor(tokens, index);
                left = Node::AddOp(Box::new(left), Box::new(right));
            }
            TokenKind::Subtract => {
                *index += 1;
                let right = parse_factor(tokens, index);
                left = Node::SubOp(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    left
}
fn parse_factor(tokens: &Vec<Token>, index: &mut usize) -> Node {
    if let Some(token) = tokens.get(*index) {
        *index += 1;
        let node = match token.kind {
            TokenKind::Number => Node::Number(token.value.parse::<f64>().unwrap()),
            TokenKind::Identifier => { 
                let id = Node::Identifier(token.value.clone());
                id
            },
            TokenKind::OpenParenthesis => {
                let node = parse_expression(tokens, index);
                if let Some(token) = tokens.get(*index) {
                    if token.kind == TokenKind::CloseParenthesis {
                        //*index += 1;
                        node
                    }
                    else {
                        panic!("Expected ')' token")
                    }
                } 
                else {
                    panic!("Unexpected end of tokens")
                }
            }
            _ => panic!("Expected number or identifier token"),
        };
        node
    } else {
        panic!("Unexpected end of tokens")
    }
}
pub trait Visitor<T> {
    fn visit_number(&mut self, node: &Node) -> T;
    fn visit_term(&mut self, node: &Node) -> T;
    fn visit_factor(&mut self, node: &Node) -> T;
    fn visit_binary_op(&mut self, node: &Node) -> T;
    fn visit_assignment(&mut self, node: &Node) -> T;
    fn visit_declaration(&mut self, node: &Node) -> T;
    fn visit_variable(&mut self, node: &Node) -> T;
}
struct PrintVisitor;
impl Visitor<()> for PrintVisitor {
    fn visit_declaration(&mut self, node: &Node) -> () {
        println!("visit_declaration: {:?}", node);
    }
    fn visit_variable(&mut self, node: &Node) -> () {
        println!("visit_variable: {:?}", node);
    }
    fn visit_number(&mut self, node: &Node) -> () {
        println!("visit_number: {:?}", node);
    }

    fn visit_term(&mut self, node: &Node) -> () {
        println!("visit_term: {:?}", node);
    }

    fn visit_factor(&mut self, node: &Node) -> () {
        println!("visit_factor: {:?}", node);
    }

    fn visit_assignment(&mut self, node: &Node) -> () {
        println!("visit_assignment: {:?}", node);
    }

    fn visit_binary_op(&mut self, node: &Node) -> () {
        println!("visit_binary_op: {:?}", node);
    }
}
