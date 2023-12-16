use crate::tokens::TokenKind;

pub enum Node {
    Float64(f64),
    Int32(i32),
    // literal, operator, literal, addition.
    Term(Box<Node>, TokenKind, Box<Node>),
    // term, operator, term, multiplication.
    Factor(Box<Node>, TokenKind, Box<Node>),
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
            Node::Float64(val) => visitor.visit_number(self),
            Node::Int32(val) => visitor.visit_number(self),
            Node::Term(lhs, op, rhs) => visitor.visit_term(self),
            Node::Factor(lhs, op, rhs) => visitor.visit_factor(self),
        }
    }
}