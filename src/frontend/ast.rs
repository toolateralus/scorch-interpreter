use super::tokens::TokenKind;
pub trait Visitor<T> {
    // Precedence 1
    fn visit_block(&mut self, node: &Node) -> T;
    fn visit_program(&mut self, node: &Node) -> T;
    
    // Precedence 2
    fn visit_number(&mut self, node: &Node) -> T;
    fn visit_string(&mut self, node: &Node) -> T;
    fn visit_identifier(&mut self, node: &Node) -> T;
    fn visit_bool(&mut self, node: &Node) -> T;
    // fn visit_array(&mut self, node: &Node) -> T;
    // fn visit_array_access(&mut self, node: &Node) -> T;
    
    // Precedence 3
    fn visit_expression(&mut self, node: &Node) -> T;
    
    // Precedence 4
    fn visit_binary_op(&mut self, node: &Node) -> T;
    fn visit_relational_expression(&mut self, node: &Node) -> T;
    fn visit_logical_expression(&mut self, node: &Node) -> T;
    
    // Precedence 5
    fn visit_not_op(&mut self, node: &Node) -> T;
    fn visit_neg_op(&mut self, node: &Node) -> T;
    
    // Precedence 6
    fn visit_assignment(&mut self, node: &Node) -> T;
    fn visit_declaration(&mut self, node: &Node) -> T;
    fn visit_function_call(&mut self, node: &Node) -> T;
    fn visit_function_decl(&mut self, node: &Node) -> T;
    fn visit_param_decl(&mut self, node: &Node) -> T;
    
    // Precedence 7
    // fn visit_repeat_stmnt(&mut self, node: &Node) -> T;
    // fn visit_break_stmnt(&mut self, node: &Node) -> T;
    // fn visit_if_stmnt(&mut self, node: &Node) -> T;
    // fn visit_else_stmnt(&mut self, node: &Node) -> T;

    // Precedence 8
    fn visit_lambda(&mut self, node: &Node) -> T;

    // Precedence 9
    fn visit_eof(&mut self, node: &Node) -> T;
}
#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Box<Node>>),
    Block(Vec<Box<Node>>),

    // literal & values
    Undefined(),
    String(String),
    Identifier(String),
    Bool(bool),
    
    // Expressions
    LogicalExpression {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },
    RelationalExpression {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },
    BinaryOperation(Box<Node>, TokenKind, Box<Node>),
    // todo: implement remainder operator.

    // todo: do the same with Unary operations :
    // we can have a special noed for these instead of
    // weaving it in with factors.
    NegOp(Box<Node>), // for unary -
    NotOp(Box<Node>), // for unary !

    Expression(Box<Node>),
    // Statements
    Assignment {
        id: String,
        expression: Box<Node>,
    },

    FunctionCall {
        id: String,
        arguments: Option<Vec<Node>>,
    },

    DeclStmt {
        target_type: String,
        id: String,
        expression: Box<Node>,
        mutable: bool,
    },
    RepeatStmnt {
        iterator_id: Option<String>,
        condition: Option<Box<Node>>,
        block: Box<Node>,
    },
    // not implemented
    IfStmnt {
        condition: Box<Node>,
        block: Box<Node>,
        else_stmnt: Option<Box<Node>>,
    },
    ElseStmnt {
        condition: Option<Box<Node>>,
        block: Box<Node>,
        else_stmnt: Option<Box<Node>>,
    },
    FnDeclStmnt {
        id: String,
        body: Box<Node>,
        params: Vec<Node>,
        return_type: String,
        mutable: bool,
    },
    ParamDeclNode {
        varname: Box<Node>,
        typename: Box<Node>,
    },
    BreakStmnt(Option<Box<Node>>),
    Array {
        typename: String,
        elements: Vec<Box<Node>>,
        init_capacity: usize,
        mutable: bool,
        elements_mutable: bool,
    },
    ArrayAccessExpr {
        id: String,
        index_expr: Box<Node>,
        expression: Option<Box<Node>>,
        assignment: bool,
    },
    Int(u64),
    Double(f64),
    DotOp {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },
    Lambda {
        params: Vec<Box<Node>>,
        block: Box<Node>,
    },
}
impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            // functions
            Node::FnDeclStmnt { .. } => visitor.visit_function_decl(self),
            Node::ParamDeclNode { .. } => visitor.visit_param_decl(self),
            Node::FunctionCall { .. } => visitor.visit_function_call(self),
            // Node::Lambda { .. } => visitor.visit_lambda(self),
            
            // control flow keywords
            // Node::RepeatStmnt { .. } => visitor.visit_repeat_stmnt(self),
            // Node::BreakStmnt(_) => visitor.visit_break_stmnt(self),
            // Node::IfStmnt { .. } => visitor.visit_if_stmnt(self),
            // Node::ElseStmnt { .. } => visitor.visit_else_stmnt(self),
            
            // arrays
            // Node::Array { .. } => visitor.visit_array(self),
            // Node::ArrayAccessExpr { .. } => visitor.visit_array_access(self),
            
            Node::Program(..) => visitor.visit_program(self),
            Node::Block(..) => visitor.visit_block(self),
            
            Node::BinaryOperation(..) => visitor.visit_binary_op(self),
            Node::Expression(..) => visitor.visit_expression(self),
            
            Node::DeclStmt { .. } => visitor.visit_declaration(self),
            Node::Assignment { .. } => visitor.visit_assignment(self),
            
            Node::Identifier(..) => visitor.visit_identifier(self),
            Node::Undefined() => visitor.visit_eof(self),
            
            Node::String(..) => visitor.visit_string(self),
            Node::Bool(..) => visitor.visit_bool(self),
            Node::Int(..) => visitor.visit_number(self),
            Node::Double(..) => visitor.visit_number(self),
            
            Node::NegOp(..) => visitor.visit_neg_op(self),
            Node::NotOp(..) => visitor.visit_not_op(self),
            
            Node::RelationalExpression { .. } => visitor.visit_relational_expression(self),
            Node::LogicalExpression { .. } => visitor.visit_logical_expression(self),
            
            
            _ => {
                dbg!(self);
                panic!("Not implemented")
            }
        }
    }
}
