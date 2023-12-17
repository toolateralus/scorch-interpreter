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

    Expression(Box<Node>),
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
    Block (Vec<Box<Node>>),
}
impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Node::Undefined() => panic!("visitor reached undefined node."),
            Node::Identifier(_key) => visitor.visit_factor(self),
            Node::Number(_value) => { visitor.visit_factor(self) },
            Node::AddOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::SubOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::MulOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::DivOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::AssignStmnt {id: _, expression: _} => visitor.visit_assignment(self),
            Node::DeclStmt { target_type, id, expression } => visitor.visit_declaration(self),
            Node::Block(statements) => visitor.visit_block(self),
            Node::Expression(root) => visitor.visit_expression(self),
        }
    }
}
pub fn parse_program(tokens: &Vec<Token>) -> Node {
    let mut index = 0;
    let program = parse_block(tokens, &mut index);
    program
}
fn parse_block(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut statements = Vec::new();
    while let Some(token) = tokens.get(*index) {
        if token.kind == TokenKind::CloseBrace {
            *index += 1;
            break;
        }
        let statement = parse_statement(tokens, index);
        statements.push(Box::new(statement));
    }
    Node::Block(statements)
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
            },
            TokenKind::Subtract => {
                *index += 1;
                let right = parse_term(tokens, index);
                left = Node::SubOp(Box::new(left), Box::new(right));
            },
            TokenKind::CloseParenthesis => {
                *index += 1;
                break;
            },
            TokenKind::Semicolon => {
                *index += 1;
                break;
            },
            TokenKind::Comma => {
                *index += 1;
                break;
            },
            _ => {
                println!("left");
                dbg!(left);
                println!("token");
                dbg!(token);
                panic!("unexpected token");
            }
        }
    }
    Node::Expression(Box::new(left))
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
                node
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
    fn visit_block(&mut self, node: &Node) -> T;
    fn visit_expression(&mut self, node: &Node) -> T;
    fn visit_identifier(&mut self, node: &Node) -> T;
}
pub struct PrintVisitor {
    pub indent: usize,
}
impl Visitor<()> for PrintVisitor {
    fn visit_block(&mut self, node: &Node) {
        println!("visit_block");
        if let Node::Block(statements) = node {
            for statement in statements {
                statement.accept(self);
            }
        } else {
            panic!("Expected Block node");
        }
    }
    fn visit_declaration(&mut self, node: &Node) -> () {
        println!("{}visit_declaration:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::DeclStmt { target_type, id, expression } = node {
            println!("{}type: {}", " ".repeat(self.indent), target_type);
            println!("{}id: {}", " ".repeat(self.indent), id);
            println!("{}expression:", " ".repeat(self.indent));
            self.indent += 2;
            expression.accept(self);
            self.indent -= 2;
        } else {
            panic!("Expected Declaration node");
        }
        self.indent -= 2;
    }
    fn visit_identifier(&mut self, node: &Node) -> () {
        println!("{}visit_identifier:", " ".repeat(self.indent));
    }
    fn visit_number(&mut self, node: &Node) -> () {
        println!("{}visit_number:", " ".repeat(self.indent));
    }
    fn visit_term(&mut self, node: &Node) -> () {
        println!("{}visit_number:", " ".repeat(self.indent));
    }
    fn visit_factor(&mut self, node: &Node) -> () {
        if let Node::Number(value) = node {
            println!("{}visit_factor: {}", " ".repeat(self.indent), value);
        } else if let Node::Identifier(id) = node {
            println!("{}visit_factor: {}", " ".repeat(self.indent), id);
        } else if let Node::Expression(root) = node {
            println!("{}visit_factor:", " ".repeat(self.indent));
            self.indent += 2;
            root.accept(self);
            self.indent -= 2;
        }
        else {
            dbg!(node);
            panic!("Expected Number or Identifier node");
        }
    }
    fn visit_assignment(&mut self, node: &Node) -> () {
        println!("{}visit_number:", " ".repeat(self.indent));
    }
    fn visit_binary_op(&mut self, node: &Node) -> () {
        println!("{}visit_number:", " ".repeat(self.indent));

        match node {
            Node::AddOp(lhs, rhs) => {
                println!("{}visit_add_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            },
            Node::SubOp(lhs, rhs) => {
                println!("{}visit_sub_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            },
            Node::MulOp(lhs, rhs) => {
                println!("{}visit_mul_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            },
            Node::DivOp(lhs, rhs) => {
                println!("{}visit_div_op:", " ".repeat(self.indent));
                self.indent += 2;
                lhs.accept(self);
                rhs.accept(self);
                self.indent -= 2;
            },
            _ => panic!("Expected binary operation node"),
        }

    }

    fn visit_expression(&mut self, node: &Node) -> () {
        println!("{}visit_expression:", " ".repeat(self.indent));
        self.indent += 2;
        if let Node::Expression(root) = node {
            root.accept(self);
        } else {
            panic!("Expected Expression node");
        }
        self.indent -= 2;
    }
}
