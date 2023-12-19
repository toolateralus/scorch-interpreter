use core::panic;
use crate::tokens::*;
pub trait Visitor<T> {
    fn visit_number(&mut self, node: &Node) -> T;
    fn visit_term(&mut self, node: &Node) -> T;
    fn visit_factor(&mut self, node: &Node) -> T;
    fn visit_eof(&mut self, node: &Node) -> T;
    fn visit_binary_op(&mut self, node: &Node) -> T;
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

    fn visit_where_stmnt(&mut self, node: &Node) -> T;
    fn visit_or_stmnt(&mut self, node: &Node) -> T;
}
#[derive(Debug)]
pub enum Node {
    // literal & values
    Undefined(),
    Number(f64),
    String(String),
    Identifier(String),
    RelationalExpression {
        lhs : Box<Node>,
        op : TokenKind, 
        rhs : Box<Node>,
    },
    BinaryOperation {
        lhs : Box<Node>,
        op : TokenKind, 
        rhs : Box<Node>,
    },
    // binary operations
    AddOp(Box<Node>, Box<Node>),
    SubOp(Box<Node>, Box<Node>),
    MulOp(Box<Node>, Box<Node>),
    DivOp(Box<Node>, Box<Node>),
    NegOp(Box<Node>), // for unary minus
    NotOp(Box<Node>), // for unary not
    // todo: implement Modulo & Unary operations.
    Expression(Box<Node>),
    // Statements
    AssignStmnt {
        id: Box<Node>,
        expression: Box<Node>,
    },
    DeclStmt {
        target_type: String,
        id: String,
        expression: Box<Node>,
    },
    WhereStmnt {
        condition: Box<Node>,
        block: Box<Node>,
        or_stmnt: Option<Box<Node>>,
    },
    OrStmnt {
        condition: Option<Box<Node>>,
        block: Box<Node>,
        or_stmnt: Option<Box<Node>>,
    },
    Block(Vec<Box<Node>>),
    Bool(bool),
    LogicalExpression { lhs: Box<Node>, op: TokenKind, rhs: Box<Node> },
}
impl Node {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Node::Undefined() => visitor.visit_eof(self),
            Node::Identifier(_key) => visitor.visit_identifier(self),
            Node::Number(_value) => visitor.visit_number(self),
            Node::AddOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::SubOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::MulOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::DivOp(_lhs, _rhs) => visitor.visit_binary_op(self),
            Node::AssignStmnt {
                id: _,
                expression: _,
            } => visitor.visit_assignment(self),
            Node::DeclStmt {
                target_type: _,
                id: _,
                expression: _,
            } => visitor.visit_declaration(self),
            Node::Block(_statements) => visitor.visit_block(self),
            Node::Expression(_root) => visitor.visit_expression(self),
            Node::String(_) => visitor.visit_string(self),
            Node::NegOp(_) => visitor.visit_neg_op(self),
            Node::NotOp(_) => visitor.visit_not_op(self),
            Node::Bool(_) => visitor.visit_bool(self),
            Node::WhereStmnt {
                condition: _,
                block: _true_block,
                or_stmnt: _,
            } => visitor.visit_where_stmnt(self),
            Node::OrStmnt {
                condition: _,
                block: _,
                or_stmnt: _,
            } => visitor.visit_or_stmnt(self),
            Node::RelationalExpression { lhs, op, rhs } => visitor.visit_relational_expression(self),
            Node::LogicalExpression { lhs, op, rhs } => visitor.visit_logical_expression(self),
            Node::BinaryOperation { lhs, op, rhs } => visitor.visit_binary_op(self),
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

    while *index < tokens.len() {
        let token = consume_newlines(index, tokens);
        if token.kind == TokenKind::CloseBrace {
            *index += 1;
            break;
        }
        let statement = parse_statement(tokens, index);

        match statement {
            Ok(node) => statements.push(Box::new(node)),
            Err(_) => {
                if token.kind == TokenKind::Newline || token.kind == TokenKind::Semicolon {
                    break; // ignore newlines.
                }
                panic!("Expected statement node");
            }
        }
    }
    Node::Block(statements)
}
fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<Node, ()> {
    if *index >= tokens.len() {
        return Err(());
    }

    let mut token = get_current(tokens, index);
    token = consume_newlines(index, tokens);

    if *index + 1 >= tokens.len() {
        return Err(()); // probably a newline
    }

    let next = tokens.get(*index + 1).unwrap();

    match token.family {
        TokenFamily::Keyword => match token.kind {
            _ => {
                dbg!(token);
                panic!("keywords are not yet implemented.");
            }
        },
        TokenFamily::Identifier => {
            // varname : type = default;
            let id = token.value.clone();
            match next.kind {
                // varname := default;
                // declaring a variable with implicit type.
                TokenKind::ColonEquals => {
                    *index += 2;

                    // varname := ^default;
                    let value = parse_expression(tokens, index);

                    Ok(Node::DeclStmt {
                        target_type: String::from("dynamic"),
                        id,
                        expression: Box::new(value),
                    })
                }
                // declaraing a variable with explicit type.
                TokenKind::Colon => {
                    *index += 2;
                    // varname :^ type = default;
                    // todo: check for valid type / builtins
                    let target_type_tkn = get_current(tokens, index);
                    let target_type = target_type_tkn.value.clone();
                    *index += 1;

                    // varname : type^ = default;

                    if let Some(token) = tokens.get(*index) {
                        assert_eq!(
                            token.kind,
                            TokenKind::Assignment,
                            "Expected identifier token"
                        );
                    } else {
                        dbg!(token);
                        panic!("expected type identifier in declaration statement");
                    }
                    *index += 1;

                    // varname : type = ^default;
                    let expression = parse_expression(tokens, index);
                    Ok(Node::DeclStmt {
                        target_type,
                        id,
                        expression: Box::new(expression),
                    })
                }
                // assigning a value to an already declared variable.
                TokenKind::Assignment => {
                    *index += 2;
                    let id = Node::Identifier(token.value.clone());
                    let expression = parse_expression(tokens, index);
                    Ok(Node::AssignStmnt {
                        id: Box::new(id),
                        expression: Box::new(expression),
                    })
                }
                _ => {
                    dbg!(token);
                    println!("Expected ':' or '=' token after Identifier,\n instead got : \n current : {:?}\n next : {:?}", token, next);
                    panic!("parser failure : check logs.");
                }
            }
        }
        TokenFamily::Operator => {
            if token.kind == TokenKind::OpenBrace {
                *index += 1;
                let block = parse_block(tokens, index);
                Ok(block)
            } else {
                dbg!(token);
                panic!("Expected brace token");
            }
        }
        _ => {
            dbg!(token);
            panic!("Expected keyword, identifier or operator token");
        }
    }
}
fn get_current<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> &'a Token {
    if let Some(token) = tokens.get(*index) {
        return token;
    } else {
        panic!("Unexpected end of tokens")
    }
}
fn consume_newlines<'a>(index: &mut usize, tokens: &'a Vec<Token>) -> &'a Token {
    let mut current = get_current(tokens, index);
    while *index + 1 < tokens.len() && current.kind == TokenKind::Newline {
        *index += 1;
        current = get_current(tokens, index);
    }
    return current;
}
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_logical_expr(tokens, index);

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::LogicalAnd | 
            TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_logical_expr(tokens, index);
                left = Node::LogicalExpression {
                    lhs : Box::new(left),
                    op : token.kind,
                    rhs : Box::new(right) 
                };
            }
            TokenKind::CloseParenthesis => {
                *index += 1;
                break;
            }
            TokenKind::Newline => {
                *index += 1;
                break;
            }
            TokenKind::Semicolon => {
                *index += 1;
                break;
            }
            TokenKind::Comma => {
                *index += 1;
                break;
            }
            
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
fn parse_logical_expr(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_relational_expr(tokens, index);
    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::LogicalAnd |
            TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_relational_expr(tokens, index);
                left = Node::LogicalExpression {
                    lhs : Box::new(left),
                    op :token.kind, 
                    rhs : Box::new(right)
                };
            }
            _ => break,
        }
    }
    left
}
fn parse_relational_expr(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_addition(tokens, index);
    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Equals |
            TokenKind::NotEquals |
            TokenKind::LessThanEquals |
            TokenKind::GreaterThanEquals |
            TokenKind::LeftAngle |
            TokenKind::RightAngle
            => {
                *index += 1;
                let right = parse_addition(tokens, index);
                left = Node::RelationalExpression {
                    lhs: Box::new(left),
                    op : token.kind,
                    rhs: Box::new(right),
                };
            }
            _ => break,
        };
    }
    left
    
}
fn parse_addition(tokens: &Vec<Token>, index: &mut usize) -> Node {
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
            }
            _ => break,
        }
    }
    left
}
fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_factor(tokens, index);
    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Multiply => {
                *index += 1;
                let right = parse_factor(tokens, index);
                left = Node::MulOp(Box::new(left), Box::new(right));
            }
            TokenKind::Divide => {
                *index += 1;
                let right = parse_factor(tokens, index);
                left = Node::DivOp(Box::new(left), Box::new(right));
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
            }
            TokenKind::String => {
                let id = Node::String(token.value.clone());
                id
            }
            TokenKind::OpenParenthesis => {
                let node = parse_expression(tokens, index);
                node
            }
            TokenKind::Subtract => {
                let node = parse_factor(tokens, index);
                Node::NegOp(Box::new(node))
            }
            TokenKind::Not => {
                let node = parse_factor(tokens, index);
                Node::NotOp(Box::new(node))
            }
            TokenKind::Bool => {
                let boolean = Node::Bool(token.value.parse::<bool>().unwrap());
                boolean
            }
            _ => panic!("Expected number or identifier token"),
        };
        node
    } else {
        panic!("Unexpected end of tokens")
    }
}