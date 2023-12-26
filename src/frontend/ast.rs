use super::tokens::TokenKind;
pub trait Visitor<T> {
    fn visit_number(&mut self, node: &Node) -> T;
    fn visit_term(&mut self, node: &Node) -> T;
    fn visit_factor(&mut self, node: &Node) -> T;
    fn visit_eof(&mut self, node: &Node) -> T;
    fn visit_binary_op(&mut self, node: &Node) -> T;
    fn visit_lambda(&mut self, node: &Node) -> T;
    fn visit_function_decl(&mut self, node: &Node) -> T;
    fn visit_param_decl(&mut self, node: &Node) -> T;
    fn visit_function_call(&mut self, node: &Node) -> T;
    fn visit_program(&mut self, node: &Node) -> T;
    fn visit_repeat_stmnt(&mut self, node: &Node) -> T;
    fn visit_break_stmnt(&mut self, node: &Node) -> T;
    fn visit_relational_expression(&mut self, node: &Node) -> T;
    fn visit_logical_expression(&mut self, node: &Node) -> T;
    // unary operations
    fn visit_not_op(&mut self, node: &Node) -> T;
    fn visit_neg_op(&mut self, node: &Node) -> T;
    fn visit_assignment(&mut self, node: &Node) -> T;
    fn visit_declaration(&mut self, node: &Node) -> T;
    fn visit_block(&mut self, node: &Node) -> T;
    fn visit_expression(&mut self, node: &Node) -> T;
    fn visit_string(&mut self, node: &Node) -> T;
    fn visit_identifier(&mut self, node: &Node) -> T;
    fn visit_bool(&mut self, node: &Node) -> T;
    fn visit_array(&mut self, node: &Node) -> T;
    fn visit_array_access(&mut self, node: &Node) -> T;
    fn visit_if_stmnt(&mut self, node: &Node) -> T;
    fn visit_else_stmnt(&mut self, node: &Node) -> T;
    fn visit_type_def(&mut self, node: &Node) -> T;
    fn visit_struct_init(&mut self, node: &Node) -> T;
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
    BinaryOperation {
        lhs: Box<Node>,
        op: TokenKind,
        rhs: Box<Node>,
    },

    // todo: implement remainder operator.
    // todo: remove the individual binary operations
    // and use the BinaryOperation node with the TokenKind
    // operator field.
    AddOp(Box<Node>, Box<Node>),
    SubOp(Box<Node>, Box<Node>),
    MulOp(Box<Node>, Box<Node>),
    DivOp(Box<Node>, Box<Node>),

    // todo: do the same with Unary operations :
    // we can have a special noed for these instead of
    // weaving it in with factors.
    NegOp(Box<Node>), // for unary -
    NotOp(Box<Node>), // for unary !

    Expression(Box<Node>),
    // Statements
    AssignStmnt {
        id: Box<Node>,
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
    Int(i32),
    Double(f64),
    DotOp { lhs: Box<Node>, op: TokenKind, rhs: Box<Node> },
    Lambda { params: Vec<Box<Node>>, block: Box<Node> },
    StructDef { id: String, block: Box<Node> },
    StructInit { id: String, args: Vec<Node> },
}
impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Node::Undefined() => visitor.visit_eof(self),
            Node::Identifier( .. ) => visitor.visit_identifier(self),
            Node::AddOp( .. ) => visitor.visit_binary_op(self),
            Node::SubOp( .. ) => visitor.visit_binary_op(self),
            Node::MulOp( .. ) => visitor.visit_binary_op(self),
            Node::DivOp( .. ) => visitor.visit_binary_op(self),
            Node::AssignStmnt { .. } => visitor.visit_assignment(self),
            Node::DeclStmt { .. } => visitor.visit_declaration(self),
            Node::Block( .. ) => visitor.visit_block(self),
            Node::Expression( .. ) => visitor.visit_expression(self),
            Node::String( .. ) => visitor.visit_string(self),
            Node::NegOp( .. ) => visitor.visit_neg_op(self),
            Node::NotOp( .. ) => visitor.visit_not_op(self),
            Node::Bool( .. ) => visitor.visit_bool(self),
            Node::IfStmnt { .. } => visitor.visit_if_stmnt(self),
            Node::ElseStmnt { .. } => visitor.visit_else_stmnt(self),
            Node::RelationalExpression { .. } => visitor.visit_relational_expression(self),
            Node::LogicalExpression { .. } => visitor.visit_logical_expression(self),
            Node::BinaryOperation { .. } => visitor.visit_binary_op(self),
            Node::FnDeclStmnt { .. } => visitor.visit_function_decl(self),
            Node::ParamDeclNode { .. } => visitor.visit_param_decl(self),
            Node::FunctionCall { .. } => visitor.visit_function_call(self),
            Node::Program( .. ) => visitor.visit_program(self),
            Node::RepeatStmnt { .. } => visitor.visit_repeat_stmnt(self),
            Node::BreakStmnt( .. ) => visitor.visit_break_stmnt(self),
            Node::Array { .. } => visitor.visit_array(self),
            Node::ArrayAccessExpr { .. } => visitor.visit_array_access(self),
            Node::Int(..) => visitor.visit_number(self),
            Node::Double(..) => visitor.visit_number(self),
            Node::DotOp { .. } => visitor.visit_binary_op(self),
            Node::Lambda { .. } => visitor.visit_lambda(self),
            Node::StructDef { .. } => visitor.visit_type_def(self),
            Node::StructInit { id: _, args: _ } => visitor.visit_struct_init(self),
        }
    }
}
