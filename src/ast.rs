use core::panic;

use crate::tokens::*;
pub trait Visitor<T> {
    fn visit_number(&mut self, node: &Node) -> T;
    fn visit_term(&mut self, node: &Node) -> T;
    fn visit_factor(&mut self, node: &Node) -> T;
    fn visit_eof(&mut self, node: &Node) -> T;
    fn visit_binary_op(&mut self, node: &Node) -> T;
    fn visit_function_decl(&mut self, node: &Node) -> T;
    fn visit_param_decl(&mut self, node: &Node) -> T;
    fn visit_function_call(&mut self, node: &Node) -> T;
    fn visit_program(&mut self, node: &Node) -> T;
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

    fn visit_if_stmnt(&mut self, node: &Node) -> T;
    fn visit_else_stmnt(&mut self, node: &Node) -> T;
}
#[derive(Debug, Clone)]
pub enum Node {
    Program(Vec<Box<Node>>),
    Block(Vec<Box<Node>>),

    // literal & values
    Undefined(),
    Number(f64),
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
    },
    ParamDeclNode {
        varname: Box<Node>,
        typename: Box<Node>,
    },
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
            Node::IfStmnt {
                condition: _,
                block: _true_block,
                else_stmnt: _,
            } => visitor.visit_if_stmnt(self),
            Node::ElseStmnt {
                condition: _,
                block: _,
                else_stmnt: _,
            } => visitor.visit_else_stmnt(self),
            Node::RelationalExpression {
                lhs: _,
                op: _,
                rhs: _,
            } => visitor.visit_relational_expression(self),
            Node::LogicalExpression {
                lhs: _,
                op: _,
                rhs: _,
            } => visitor.visit_logical_expression(self),
            Node::BinaryOperation {
                lhs: _,
                op: _,
                rhs: _,
            } => visitor.visit_binary_op(self),
            Node::FnDeclStmnt {
                id: _,
                body: _,
                params: _,
                return_type: _,
            } => visitor.visit_function_decl(self),
            Node::ParamDeclNode {
                varname: _,
                typename: _,
            } => visitor.visit_param_decl(self),
            Node::FunctionCall {
                id: _,
                arguments: _,
            } => visitor.visit_function_call(self),
            Node::Program(_statements) => visitor.visit_program(self),
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
fn consume_normal_expr_delimiter(tokens: &Vec<Token>, index: &mut usize) {
    let current = get_current(tokens, index).kind;
    match current {
        TokenKind::OpenBrace | TokenKind::Comma => {
            dbg!(current);
            panic!("expected newline or ) token");
        }
        TokenKind::Newline => {
            *index += 1;
        }
        TokenKind::CloseParenthesis => {
            *index += 1;
        }
        _ => {
            // continue
        }
    }
}

// ########################################
// START PARSER FUNCTION HIERARCHY
// TOP -> BOTTOM
// GREATEST -> LEAST PRECEDENCE
// ########################################
pub fn parse_program(tokens: &Vec<Token>) -> Node {
    let mut index = 0;
    let mut statements = Vec::new();
    while index < tokens.len() {
        let token = consume_newlines(&mut index, tokens);
        if token.kind == TokenKind::Eof {
            break;
        }
        let statement = parse_statement(tokens, &mut index);

        match statement {
            Ok(node) => statements.push(Box::new(node)),
            Err(_) => {
                if token.kind == TokenKind::Newline {
                    break; // ignore newlines.
                }
                panic!("Expected statement node");
            }
        }
    }
    Node::Program(statements)
}

fn parse_block(tokens: &Vec<Token>, index: &mut usize) -> Node {
    *index += 1;
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
                if token.kind == TokenKind::Newline {
                    break; // ignore newlines.
                }
                println!("Block encountered unexpected token:");
                dbg!(&token);
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

    let token = consume_newlines(index, tokens);

    if *index + 1 >= tokens.len() {
        return Err(()); // probably a newline
    }

    let next = tokens.get(*index + 1).unwrap();

    match token.family {
        TokenFamily::Keyword => match token.kind {
            TokenKind::If => {
                let statement = parse_if_else(tokens, index);
                Ok(statement)
            }
            TokenKind::Else => {
                dbg!(token);
                panic!("else statements must follow an if.");
            }
            _ => {
                dbg!(token);
                panic!("keywords are not yet implemented.");
            }
        },
        TokenFamily::Identifier => {
            // varname : type = default;
            let id = token.value.clone();
            
            match next.kind {
                TokenKind::OpenParenthesis => {
                    *index += 1;
                    let arguments = parse_arguments(tokens, index);
                    let node = Node::FunctionCall {
                        id: token.value.clone(),
                        arguments: Option::Some(arguments),
                    };
                    Ok(node)
                }
                // varname := default;
                // declaring a variable with implicit type.
                TokenKind::ColonEquals => {
                    *index += 2; // skip id, := tokens
                    
                    // function defintion : implicit, parameterless
                    // example : foo := {...}
                    if get_current(tokens, index).kind == TokenKind::OpenBrace {
                        let body = parse_block(tokens, index);
                        //dbg!(&body);
                        let node = Node::FnDeclStmnt {
                            id,
                            body: Box::new(body),
                            params: Vec::new(),
                            return_type: String::from("dynamic"),
                        };
                        return Ok(node);
                    }

                    // function definition : implicit, with parameters
                    // example : foo := (a, b) {...}
                    if get_current(tokens, index).kind == TokenKind::OpenParenthesis {
                        // skip ahead the possible identifier & get to a colon,
                        // if this is a function definition
                        *index += 2;
                        if get_current(tokens, index).kind == TokenKind::Colon {
                            *index -= 2; // go back to the a :
                            
                            let params = parse_parameters(tokens, index);
                            let body = parse_block(tokens, index);
                            let node = Node::FnDeclStmnt {
                                id,
                                body: Box::new(body),
                                params,
                                return_type: String::from("dynamic"),
                            };
                            return Ok(node);
                        }
                        *index -= 1;
                    }
                    // implicit variable declaration
                    let value = parse_expression(tokens, index);
                    consume_normal_expr_delimiter(tokens, index);

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
                    consume_normal_expr_delimiter(tokens, index);
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
                    consume_normal_expr_delimiter(tokens, index);
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

fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_logical_expr(tokens, index);

    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_logical_expr(tokens, index);
                left = Node::LogicalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            // these 4 token kinds are expression delimiters, but
            // the tokens are expected to be consumed by the caller of this function.
            TokenKind::CloseParenthesis => {
                break;
            }
            TokenKind::OpenBrace => {
                break;
            }
            TokenKind::Newline => {
                break;
            }
            TokenKind::Comma => {
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
            TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_relational_expr(tokens, index);
                left = Node::LogicalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
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
            TokenKind::Equals
            | TokenKind::NotEquals
            | TokenKind::LessThanEquals
            | TokenKind::GreaterThanEquals
            | TokenKind::LeftAngle
            | TokenKind::RightAngle => {
                *index += 1;
                let right = parse_addition(tokens, index);
                left = Node::RelationalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
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
                if let Some(token) = tokens.get(*index) {
                    if token.kind != TokenKind::CloseParenthesis {
                        dbg!(token);
                        panic!("Expected close parenthesis token");
                    }
                    *index += 1;
                }
                node
            }
            TokenKind::Subtract => {
                let node = parse_factor(tokens, index);

                if let Node::NegOp(_node) = node {
                    panic!("Double not operations are not allowed");
                }

                Node::NegOp(Box::new(node))
            }
            TokenKind::Not => {
                let node = parse_factor(tokens, index);

                if let Node::NotOp(_node) = node {
                    panic!("Double not operations are not allowed");
                }

                Node::NotOp(Box::new(node))
            }
            TokenKind::Bool => {
                let boolean = Node::Bool(token.value.parse::<bool>().unwrap());
                boolean
            }
            _ => {
                dbg!(token);
                panic!("Expected number or identifier token");
            }
        };
        node
    } else {
        panic!("Unexpected end of tokens")
    }
}
// ########################################
// END PARSER FUNCTION HIERARCHY
// ########################################

fn parse_parameters(tokens: &Vec<Token>, index: &mut usize) -> Vec<Node> {
    *index += 1; // discard open_paren

    let mut params = Vec::new();

    loop {
        let mut token = get_current(tokens, index);

        if token.kind == TokenKind::CloseParenthesis {
            *index += 1;
            break;
        }
        
        // parsing varname
        // ^varname: Typename
        if token.family != TokenFamily::Identifier {
            panic!("Expected variable name in parameter declaration");
        }

        let varname = parse_factor(tokens, index);
        
        token = get_current(tokens, index);
        //parsing colon
        // varname^: Typename
        match token.kind {
            TokenKind::ColonEquals => {
                panic!("implicit default value & parameter type not yet implement")
            }
            TokenKind::Colon => {
                // got our valid case.
                *index += 1;
            }
            _ => {
                dbg!(token);
                panic!("Expected colon token after variable name in parameter declaration got");
            }
        }

        // parsing type
        // varname: ^Typename
        let typename = parse_factor(tokens, index);

        // consume comma if there is one.
        if get_current(tokens, index).kind == TokenKind::Comma {
            *index += 1;
        }

        let param_decl_node = Node::ParamDeclNode {
            varname: Box::new(varname),
            typename: Box::new(typename),
        };

        params.push(param_decl_node);
    }

    params
}
fn parse_arguments(tokens: &Vec<Token>, index: &mut usize) -> Vec<Node> {
    *index += 1; // discard open_paren

    let mut args = Vec::new();

    while let Some(token) = tokens.get(*index) {
        // paramless.
        if token.kind == TokenKind::CloseParenthesis {
            *index += 1;
            break;
        }
        // accumulate parameter expressions
        let arg = parse_expression(tokens, index);
        // skip commas
        if get_current(tokens, index).kind == TokenKind::Comma {
            *index += 1;
        }
        args.push(arg);
    }
    args
}
fn parse_if_else(tokens: &Vec<Token>, index: &mut usize) -> Node {
    *index += 1; // discard 'if'
    let if_condition = parse_expression(tokens, index);

    if get_current(tokens, index).kind != TokenKind::OpenBrace {
        dbg!(get_current(tokens, index));
        dbg!(if_condition);
        panic!("If expected open brace after condition");
    }

    *index += 1; // skip open brace

    let if_block = parse_block(tokens, index);

    let else_or_end = consume_newlines(index, tokens);

    // if, no else.
    if else_or_end.kind == TokenKind::Else {
        let else_node = parse_else(tokens, index);
        return Node::IfStmnt {
            condition: Box::new(if_condition),
            block: Box::new(if_block),
            else_stmnt: Option::Some(Box::new(else_node)),
        };
    } else {
        // an 'if' with no 'else.
        return Node::IfStmnt {
            condition: Box::new(if_condition),
            block: Box::new(if_block),
            else_stmnt: Option::None,
        };
    }
}
fn parse_else(tokens: &Vec<Token>, index: &mut usize) -> Node {
    *index += 1; // discard 'else'

    let _ = consume_newlines(index, tokens);

    // if else with no comparison -> if ... {} else {}
    if get_current(tokens, index).kind == TokenKind::OpenBrace {
        let else_block = parse_block(tokens, index);

        // Check for another else after this block
        if get_current(tokens, index).kind == TokenKind::Else {
            let nested_else = parse_else(tokens, index);
            return Node::ElseStmnt {
                condition: Option::None,
                block: Box::new(else_block),
                else_stmnt: Option::Some(Box::new(nested_else)),
            };
        } else {
            return Node::ElseStmnt {
                condition: Option::None,
                block: Box::new(else_block),
                else_stmnt: Option::None,
            };
        }
    }
    // if else with comparison -> if ... {} else ... {}
    else {
        let else_condition = parse_expression(tokens, index);
        let cur = get_current(tokens, index);

        match cur.kind {
            TokenKind::OpenBrace | TokenKind::CloseParenthesis => {
                *index += 1; // skip open brace
            }
            _ => {
                // continue.
            }
        }

        let else_block = parse_block(tokens, index);

        if get_current(tokens, index).kind == TokenKind::Else {
            let nested_else = parse_else(tokens, index);
            return Node::ElseStmnt {
                condition: Option::Some(Box::new(else_condition)),
                block: Box::new(else_block),
                else_stmnt: Option::Some(Box::new(nested_else)),
            };
        } else {
            return Node::ElseStmnt {
                condition: Option::Some(Box::new(else_condition)),
                block: Box::new(else_block),
                else_stmnt: Option::None,
            };
        }
    }
}
