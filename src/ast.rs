use crate::tokens::*;

#[derive(Debug)]
pub enum Node {
    Undefined(),
    Number(f64),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Identifier(String),
    Assignment {
        id: Box<Node>,
        expression : Box<Node>,
    }
}

impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Node::Undefined() => panic!("visitor reached undefined node."),
            Node::Number(_value) => { visitor.visit_number(self) },
            Node::Add(_lhs, _rhs) => visitor.visit_term(self),
            Node::Subtract(_lhs, _rhs) => visitor.visit_term(self),
            Node::Multiply(_lhs, _rhs) => visitor.visit_factor(self),
            Node::Divide(_lhs, _rhs) => visitor.visit_factor(self),
            Node::Assignment {id: _, expression: _} => visitor.visit_assignment(self),
            Node::Identifier(_key) => visitor.visit_variable(self),
        }
    }
}
pub fn parse(tokens: &Vec<Token>) -> Node {
    let mut index = 0;
    parse_expression(tokens, &mut index)
}
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_term(tokens, index);

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Add => {
                *index += 1;
                let right = parse_term(tokens, index);
                left = Node::Add(Box::new(left), Box::new(right));
            }
            TokenKind::Subtract => {
                *index += 1;
                let right = parse_term(tokens, index);
                left = Node::Subtract(Box::new(left), Box::new(right));
            }
            _ => break,
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
                left = Node::Multiply(Box::new(left), Box::new(right));
            }
            TokenKind::Divide => {
                *index += 1;
                let right = parse_addition(tokens, index);
                left = Node::Divide(Box::new(left), Box::new(right));
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
                left = Node::Add(Box::new(left), Box::new(right));
            }
            TokenKind::Subtract => {
                *index += 1;
                let right = parse_factor(tokens, index);
                left = Node::Subtract(Box::new(left), Box::new(right));
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
                let mut node = Node::Undefined();
                // foo = 10;
                if tokens.get(*index + 1).unwrap().kind == TokenKind::Assignment {
                    *index += 1;
                    let value = parse_expression(tokens, index);
                    node = Node::Assignment {
                        id: Box::new(node),
                        expression: Box::new(value),
                    };
                } else {
                    
                }
                node
            },
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
    fn visit_variable(&mut self, node: &Node) -> T;
}
struct PrintVisitor;
impl Visitor<()> for PrintVisitor {
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
