use super::{
    ast::Node,
    tokens::{Token, TokenFamily, TokenKind},
};

// function helpers
pub fn parse_parameters(tokens: &Vec<Token>, index: &mut usize) -> Vec<Node> {
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

        let varname = parse_operand(tokens, index);

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
        let typename = parse_operand(tokens, index);

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
pub fn parse_arguments(tokens: &Vec<Token>, index: &mut usize) -> Vec<Node> {
    *index += 1; // discard open_paren

    let mut args = Vec::new();

    loop {
        let token = get_current(tokens, index);
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
pub fn prs_fn_decl(
    params: &Vec<Node>,
    tokens: &Vec<Token>,
    index: &mut usize,
    id: &String,
    return_type: String,
    mutable: bool,
) -> Option<Result<Node, ()>> {
    let token = get_current(tokens, index);
    let kind = token.kind;
    if kind == TokenKind::OpenCurly {
        let body = parse_block(tokens, index);
        let node = Node::FnDeclStmnt {
            id: id.clone(),
            body: Box::new(body),
            params: params.clone(),
            return_type,
            mutable,
        };
        return Some(Ok(node));
    }
    None
}
pub fn create_default_value_for_type(target_type: &String, mutable: bool) -> Node {
    let default_value_expression = match target_type.as_str() {
        "Double" => Node::Expression(Box::new(Node::Double(0.0))),
        "Int" => Node::Expression(Box::new(Node::Int(0))),
        "String" => Node::Expression(Box::new(Node::String(String::from("")))),
        "Bool" => Node::Expression(Box::new(Node::Bool(false))),

        "Array" => {
            let elements = Vec::new();
            let init_capacity = elements.len();
            let typename = String::from("Dynamic");
            let elements_mutable = mutable;
            Node::Expression(Box::new(new_array(
                typename,
                init_capacity,
                elements,
                mutable,
                elements_mutable,
            )))
        }
        _ => Node::Expression(Box::new(Node::Undefined())),
    };
    default_value_expression
}
fn parse_fn_call(index: &mut usize, tokens: &Vec<Token>, token: &String) -> Result<Node, ()> {
    let arguments = parse_arguments(tokens, index);
    let node = Node::FunctionCall {
        id: token.clone(),
        arguments: Option::Some(arguments),
    };
    Ok(node)
}

// arrays
pub fn new_array(
    typename: String,
    init_capacity: usize,
    elements: Vec<Box<Node>>,
    mutable: bool,
    elements_mutable: bool,
) -> Node {
    Node::Array {
        typename,
        init_capacity,
        elements,
        mutable,
        elements_mutable, // todo: how do we want to qualify this?
    }
}
pub fn parse_array_initializer(tokens: &Vec<Token>, index: &mut usize) -> Vec<Box<Node>> {
    let mut args = Vec::new();
    loop {
        let token = get_current(tokens, index);
        // paramless.
        if token.kind == TokenKind::CloseBracket {
            *index += 1;
            break;
        }
        // accumulate parameter expressions
        let arg = parse_expression(tokens, index);

        // skip commas
        if get_current(tokens, index).kind == TokenKind::Comma {
            *index += 1;
        }
        args.push(Box::new(arg));
    }
    args
}
pub fn parse_array_access(index: &mut usize, tokens: &Vec<Token>, id: &str) -> Result<Node, ()> {
    let accessor = parse_expression(tokens, index);
    let mut token = get_current(tokens, index);

    if token.kind == TokenKind::CloseBracket {
        *index += 1; // move past ]
        token = get_current(tokens, index);
    }

    if token.kind == TokenKind::Newline {
        *index += 1; // move past \n
        _ = consume_newlines(index, tokens);
    }

    token = get_current(tokens, index);

    let mut node = Node::ArrayAccessExpr {
        id: id.to_string(),
        index_expr: Box::new(accessor),
        expression: None,
        assignment: false,
    };

    if token.kind != TokenKind::Assignment {
        return Ok(node);
    }

    match token.kind {
        TokenKind::Assignment => {
            *index += 1;
            if let Node::ArrayAccessExpr {
                id,
                index_expr,
                expression: _,
                assignment: _,
            } = node
            {
                node = Node::ArrayAccessExpr {
                    id,
                    index_expr,
                    expression: Option::Some(Box::new(parse_expression(tokens, index))),
                    assignment: true,
                };
            }
            Ok(node)
        }
        _ => Err(()),
    }
}
pub fn get_current<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> &'a Token {
    if let Some(token) = tokens.get(*index) {
        return token;
    } else {
        panic!("Unexpected end of tokens")
    }
}
pub fn consume_newlines<'a>(index: &mut usize, tokens: &'a Vec<Token>) -> &'a Token {
    let mut current = get_current(tokens, index);
    while *index + 1 < tokens.len() && current.kind == TokenKind::Newline {
        *index += 1;
        current = get_current(tokens, index);
    }
    return current;
}
pub fn consume_normal_expr_delimiter(tokens: &Vec<Token>, index: &mut usize) {
    let current = get_current(tokens, index).kind;
    match current {
        TokenKind::OpenCurly | TokenKind::Comma => {
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

// keywords
fn parse_repeat_stmnt(next: &Token, index: &mut usize, tokens: &Vec<Token>) -> Result<Node, ()> {
    // style::
    // repeat i < 200 {...}
    if next.family == TokenFamily::Identifier {
        let id = next.value.clone();
        *index += 1; // skip repeat, leaev identifier in expression.
        let condition = parse_expression(tokens, index);
        let block = parse_block(tokens, index);
        let node = Node::RepeatStmnt {
            iterator_id: Option::Some(id),
            condition: Option::Some(Box::new(condition)),
            block: Box::new(block),
        };
        return Ok(node);
    }

    *index += 1; // skip repeat
                 // style::
                 // repeat {... }
    let block = parse_block(tokens, index);

    //*index += 1;

    Ok(Node::RepeatStmnt {
        iterator_id: Option::None,
        condition: Option::None,
        block: Box::new(block),
    })
}
fn parse_if_else(tokens: &Vec<Token>, index: &mut usize) -> Node {
    *index += 1; // discard 'if'
    let if_condition = parse_expression(tokens, index);

    if get_current(tokens, index).kind != TokenKind::OpenCurly {
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
    if get_current(tokens, index).kind == TokenKind::OpenCurly {
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
            TokenKind::OpenCurly | TokenKind::CloseParenthesis => {
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

// declarations
fn parse_decl(
    token: &Token,
    index: &mut usize,
    tokens: &Vec<Token>,
    mutable: bool,
) -> Result<Node, ()> {
    // varname : type = default;
    let id = token.value.clone();
    
    *index += 1;
    
    let operator = get_current(tokens, index);
    
    match operator.kind {
        // varname := default;
        // declaring a variable with implicit type.
        TokenKind::ColonEquals => parse_implicit_decl(index, tokens, &id, mutable),
        // declaraing a variable with explicit type.
        TokenKind::Colon => parse_explicit_decl(index, tokens, token, id, mutable),
        // assigning a value to an already declared variable.
        TokenKind::Assignment => {
            *index += 1;
            let id = Node::Identifier(token.value.clone());
            let expression = parse_expression(tokens, index);
            consume_normal_expr_delimiter(tokens, index);
            Ok(Node::AssignStmnt {
                id: Box::new(id),
                expression: Box::new(expression),
            })
        }
        TokenKind::OpenBracket => {
            *index += 1; // discard [
            Ok(parse_array_access(index, tokens, token.value.as_str()).unwrap())
        }

        // function call
        TokenKind::OpenParenthesis => {
            // silly mode. extracting functions results in these super stupid types like Result<Node, ()>
            // instead of using an Option. why.
            let Ok(node) = parse_fn_call(index, tokens, &token.value.clone()) else {
                panic!("Expected function call node");
            };
            Ok(node)
        }

        _ => {
            dbg!(token);
            println!("failed to parse declaration statement. expected ':', ':=', '=', or '('. \n instead got : \n current : {:?}\n next : {:?}", token, operator);
            panic!("parser failure : check logs.");
        }
    }
}
fn parse_implicit_decl(
    index: &mut usize,
    tokens: &Vec<Token>,
    id: &String,
    mutable: bool,
) -> Result<Node, ()> {
    *index += 1;

    if let Some(value) = parse_function_decl_stmnt(tokens, index, id, mutable) {
        return value;
    }

    if get_current(tokens, index).kind == TokenKind::Newline {
        let _token = consume_newlines(index, tokens);
    }

    // implicit variable declaration
    let value = parse_expression(tokens, index);
    consume_normal_expr_delimiter(tokens, index);

    Ok(Node::DeclStmt {
        target_type: String::from("Dynamic"),
        id: id.clone(),
        expression: Box::new(value),
        mutable,
    })
}
fn parse_function_decl_stmnt(
    tokens: &Vec<Token>,
    index: &mut usize,
    id: &String,
    mutable: bool,
) -> Option<Result<Node, ()>> {
    if get_current(tokens, index).kind == TokenKind::OpenCurly {
        let body = parse_block(tokens, index);
        //dbg!(&body);
        let node = Node::FnDeclStmnt {
            id: id.clone(),
            body: Box::new(body),
            params: Vec::new(),
            return_type: String::from("Dynamic"),
            mutable,
        };
        return Some(Ok(node));
    }
    // function defintion : implicit, parameterless
    // example : foo := {...}

    // function definition : implicit, with parameters
    // example : foo := (a, b) {...}
    if get_current(tokens, index).kind == TokenKind::OpenParenthesis {
        // skip ahead the possible identifier & get to a colon,
        // if this is a function definition
        let mut temp_index = *index + 2;
        if get_current(tokens, &mut temp_index).kind == TokenKind::Colon {
            let params = parse_parameters(tokens, index);
            let body = parse_block(tokens, index);
            let node = Node::FnDeclStmnt {
                id: id.clone(),
                body: Box::new(body),
                params,
                return_type: String::from("Dynamic"),
                mutable,
            };
            return Some(Ok(node));
        }
    }
    None
}
fn parse_explicit_decl(
    index: &mut usize,
    tokens: &Vec<Token>,
    _token: &Token,
    id: String,
    mutable: bool,
) -> Result<Node, ()> {
    // skip id token
    *index += 1;

    // varname :^ type = default;
    // todo: check for valid type / builtins
    let target_type_tkn = get_current(tokens, index);
    let target_type = target_type_tkn.value.clone();

    if target_type == "Fn" {
        *index += 1;
        let params = parse_parameters(tokens, index);

        // function explict return type function, explicit args.
        // foo : (a : String) -> String = {}
        if get_current(tokens, index).kind == TokenKind::Arrow {
            *index += 1;

            if get_current(tokens, index).kind != TokenKind::Identifier {
                dbg!(get_current(tokens, index));
                panic!("Expected type identifier");
            }

            let cur = get_current(tokens, index);

            if cur.kind != TokenKind::Identifier {
                dbg!(get_current(tokens, index));
                panic!("Expected type identifier");
            }

            let return_type = cur.value.clone();

            *index += 1;

            let Some(val) = prs_fn_decl(
                &params,
                tokens,
                index,
                &id,
                return_type.to_string(),
                mutable,
            ) else {
                dbg!(get_current(tokens, index));
                panic!("Expected function body");
            };

            let Ok(fn_def) = &val else {
                panic!("Expected function body");
            };

            *index += 1;

            return Ok(fn_def.clone());
        }
    }

    *index += 1;

    // varname : type^ = default;

    let token = get_current(tokens, index);

    // varname : type
    // uninitialized ((default for now))
    if token.kind == TokenKind::Newline {
        *index += 1;

        let default_value_expression = create_default_value_for_type(&target_type, mutable);

        return Ok(Node::DeclStmt {
            target_type,
            id,
            expression: Box::new(default_value_expression),
            mutable,
        });
    }

    *index += 1;

    // varname : type = ^default;

    let expression = parse_expression(tokens, index);
    consume_normal_expr_delimiter(tokens, index);
    Ok(Node::DeclStmt {
        target_type,
        id,
        expression: Box::new(expression),
        mutable,
    })
}

fn parse_lambda(tokens: &Vec<Token>, index: &mut usize) -> Node {
    
    if get_current(tokens, index).kind == TokenKind::LogicalOr {
        *index += 1;
        
        if get_current(tokens, index).kind == TokenKind::Lambda {
            *index += 1;
        } else {
            panic!("Expected lambda token");
        }
        
        let block = parse_block(tokens, index);
        
        Node::Lambda {
            params: Vec::new(),
            block :Box::new(block) 
        }
    } else {
        *index += 1;
        let mut params = Vec::new();
        loop {
            let token = get_current(tokens, index);
            
            if token.kind == TokenKind::Pipe || token.kind == TokenKind::Lambda {
                *index += 1;
                break;
            }
                
            let expr = Box::new(parse_expression(tokens, index));
            
            params.push(expr);
        }
        if get_current(tokens, index).kind == TokenKind::Lambda {
            *index += 1;
        } else {
            dbg!(params);
            panic!("Expected lambda token");
        }
                    
        let block = parse_block(tokens, index);
        
        Node::Lambda {
            params,
            block: Box::new(block),
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
                if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
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
        if token.kind == TokenKind::CloseCurly {
            *index += 1;
            break;
        }
        let statement = parse_statement(tokens, index);

        match statement {
            Ok(node) => statements.push(Box::new(node)),
            Err(_) => {
                if token.kind == TokenKind::Newline || token.kind == TokenKind::Eof {
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
    
    let first = consume_newlines(index, tokens);
    
    if *index + 1 >= tokens.len() {
        return Err(()); // probably a newline
    }

    let second = tokens.get(*index + 1).unwrap();
    
    // NOTE:: next is ahead one and must be discarded.
    // NOTE:: token is the current, but must also be discarded.
    // any branch of this must move the index forward at least once.
    match first.family {
        TokenFamily::Keyword => match first.kind {
            TokenKind::Const => {
                // consume 'const'
                *index += 1;
                let varname = second;
                match parse_decl(varname, index, tokens, false) {
                    Ok(node) => Ok(node),
                    Err(_) => {
                        dbg!(first);
                        panic!("Expected declaration statement");
                    }
                }
            }
            TokenKind::Var => {
                // consume 'const'
                *index += 1;
                let varname = second;
                match parse_decl(varname, index, tokens, true) {
                    Ok(node) => Ok(node),
                    Err(_) => {
                        dbg!(first);
                        panic!("Expected declaration statement");
                    }
                }
            }
            TokenKind::Break => {
                *index += 1; // discard break
                if second.kind == TokenKind::Newline {
                    Ok(Node::BreakStmnt(Option::None))
                } else if second.kind != TokenKind::CloseCurly {
                    let value = parse_expression(tokens, index);
                    Ok(Node::BreakStmnt(Option::Some(Box::new(value))))
                } else {
                    panic!("break statements must be followed by a newline or a return value.");
                }
            }
            TokenKind::Repeat => parse_repeat_stmnt(second, index, tokens),
            TokenKind::If => {
                let statement = parse_if_else(tokens, index);
                Ok(statement)
            }
            TokenKind::Else => {
                dbg!(first);
                panic!("else statements must follow an if.");
            }
            _ => {
                dbg!(first);
                panic!("keyword is likely not yet implemented.");
            }
        },
        TokenFamily::Identifier => {
            match second.kind {
                TokenKind::ColonEquals |
                TokenKind::Colon |
                TokenKind::Assignment => {
                    return parse_decl(first, index, tokens, false);
                }
                _ => {
                    let node = parse_expression(tokens, index);
                    Ok(node)
                }
            } 
        }
        TokenFamily::Operator |
        TokenFamily::Value => {
            Ok(parse_expression(tokens, index))
        }
        _ => {
            dbg!(first);
            panic!("Expected keyword, identifier or operator token");
        }
    }
}
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let mut left = parse_logical_expr(tokens, index);
    loop {
        let token = get_current(tokens, index);
        match token.kind {
            TokenKind::LogicalAnd |
            TokenKind::LogicalOr => {
                *index += 1;
                let right = parse_logical_expr(tokens, index);
                left = Node::LogicalExpression {
                    lhs: Box::new(left),
                    op: token.kind,
                    rhs: Box::new(right),
                };
            }
            // these 5 token kinds are expression delimiters, but
            // the tokens are expected to be consumed by the caller of this function.
            TokenKind::CloseParenthesis
            | TokenKind::CloseBracket
            | TokenKind::OpenCurly
            | TokenKind::Pipe
            | TokenKind::Newline
            | TokenKind::Comma
            | TokenKind::Eof => {
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
    let mut left = parse_unary(tokens, index);
    while let Some(token) = tokens.get(*index) {
        match token.kind {
            TokenKind::Multiply => {
                *index += 1;
                let right = parse_unary(tokens, index);
                left = Node::MulOp(Box::new(left), Box::new(right));
            }
            TokenKind::Divide => {
                *index += 1;
                let right = parse_unary(tokens, index);
                left = Node::DivOp(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }
    left
}

fn parse_unary(tokens : &Vec<Token>, index: &mut usize) -> Node {
    let op = get_current(tokens, index);
    match op.kind {
        TokenKind::Subtract => {
            *index += 1;
            let node = parse_compound(tokens, index);
            if let Node::NegOp(_node) = node {
                panic!("Double not operations are not allowed");
            }
            Node::NegOp(Box::new(node))
        }
        TokenKind::Not => {
            *index += 1;
            let node = parse_compound(tokens, index);
            if let Node::NotOp(_node) = node {
                panic!("Double not operations are not allowed");
            }
            Node::NotOp(Box::new(node))
        }
        _ => {
            parse_compound(tokens, index)
        }
    }
}

fn parse_compound(tokens: &Vec<Token>, index: &mut usize) -> Node {
    let left = parse_operand(tokens, index);
    let op = get_current(tokens, index);
    match op.kind {
        TokenKind::OpenParenthesis => {
            if let Node::Identifier(id) = &left {   
                match parse_fn_call(index, tokens, &id) {
                    Ok(node) => {
                        node
                    }
                    Err(_) => {
                        panic!("Expected function call node");
                    }
                }
            } else {
                panic!("Expected function call node");
            }
        }
        TokenKind::Dot => {
            *index += 1; // consume '.' operator.
            Node::DotOp {
                lhs: Box::new(left),
                op: TokenKind::Dot,
                rhs: Box::new(parse_compound(tokens, index)),
            }
        }
        
        TokenKind::OpenBracket => {
            // id[]
            if let Node::Identifier(id) = left {
                *index += 1; // move past [
                match parse_array_access(index, tokens, &id) {
                    Ok(node) => {
                        node
                    }
                    Err(_) => {
                        panic!("Expected array access node");
                    }
                }
            }
            else {
                dbg!(left);
                panic!("Expected array access node");
            }
        }
        _ => {
            left
        }
    }
    
    
}

fn parse_operand(tokens: &Vec<Token>, index: &mut usize) -> Node {
    if let Some(token) = tokens.get(*index) {
        *index += 1;
        let node = match token.kind {
            TokenKind::Number => {
                let int = token.value.parse::<i32>();
                let float = token.value.parse::<f64>();
                
                if int.is_ok() {
                    return Node::Int(int.unwrap());
                } else if float.is_ok() {
                    return Node::Double(float.unwrap());
                } else {
                    dbg!(token);
                    panic!("Expected number token");
                }
            }
            TokenKind::Identifier => {
                let id = Node::Identifier(token.value.clone());
                id
            }
            TokenKind::String => {
                let id = Node::String(token.value.clone());
                id
            }
            TokenKind::OpenBracket => {
                let init = parse_array_initializer(tokens, index);
                
                // todo: this is a hack. we need to know if the array is mutable or not by normal means.
                let array_mutable = true;
                let elements_mutable = false;    
                return new_array("Array".to_string(), init.len(), init.clone(), array_mutable, elements_mutable);
            }
            TokenKind::LogicalOr => {
                let lambda = parse_lambda(tokens, index);
                lambda
            }
            TokenKind::Pipe => {
                parse_lambda(tokens, index)
            }
            TokenKind::OpenParenthesis => {
                // todo: add value tuples maybe an epic syntax
                
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
            TokenKind::Bool => {
                let boolean = Node::Bool(token.value.parse::<bool>().unwrap());
                boolean
            }
            // todo: add a way to have a set of keywords be also operands
            TokenKind::Repeat => {
                let next = get_current(tokens, index);
                let stmnt = parse_repeat_stmnt(next, index, tokens);
                stmnt.unwrap()
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
