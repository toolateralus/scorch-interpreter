use crate::tokens::*;

#[derive(Debug)]
pub enum Node {
    Number(f64),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Assignment(String, Box<Node>),
}
pub trait Visitor<T> {
    fn visit_number(&mut self, node: &Node) -> T;
    fn visit_term(&mut self, node: &Node) -> T;
    fn visit_factor(&mut self, node: &Node) -> T;
    fn visit_binary_op(&mut self, node: &Node) -> T;
}
impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Node::Number(_) => visitor.visit_number(self),
            Node::Add(_, _) => visitor.visit_term(self),
            Node::Subtract(_, _) => visitor.visit_term(self),
            Node::Multiply(_, _) => visitor.visit_factor(self),
            Node::Divide(_, _) => visitor.visit_factor(self),
            Node::Assignment(_, _) => visitor.visit_binary_op(self),
        }
    }
}
struct PrintVisitor;
impl Visitor<()> for PrintVisitor {
    fn visit_number(&mut self, node: &Node) -> () {
        dbg!(node);
        ()
    }

    fn visit_term(&mut self, node: &Node) -> () {
        dbg!(node);
        ()
    }

    fn visit_factor(&mut self, node: &Node) -> () {
        dbg!(node);
        ()
    }

    fn visit_binary_op(&mut self, node: &Node) -> () {
        dbg!(node);
        ()
    }
}


fn parse(tokens: &Vec<Token>) -> Node {
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
    let left = parse_factor(tokens, index);

    while let Some(token) = tokens.get(*index) {
        match token.token_type {
            TokenType::Multiply => {
                *index += 1;
                let right = parse_factor(tokens, index);
                left = Node::Multiply(Box::new(left), Box::new(right));
            }
            TokenType::Divide => {
                *index += 1;
                let right = parse_factor(tokens, index);
                left = Node::Divide(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }

    left
}

fn parse_factor(tokens: &Vec<Token>, index: &mut usize) -> Node {
    if let Some(token) = tokens.get(*index) {
        *index += 1;
        match token.kind {
            TokenKind::Number => Node::Number(1.000),
        }
    } else {
        panic!("Unexpected end of tokens")
    }
}

fn parse_identifier(tokens: &Vec<Token>, index: &mut usize) -> String {
    if let Some(token) = tokens.get(*index) {
        *index += 1;
        match token.kind {
            TokenKind::Extract => token.value.clone(),
            _ => panic!("Expected identifier token"),
        }
    } else {
        panic!("Unexpected end of tokens")
    }
}