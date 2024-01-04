use super::context::Context;
use super::standard_functions::StandardFunction;
use super::typechecker::*;
use super::types::*;
use scorch_parser::ast::*;
use scorch_parser::lexer::*;
use std::cell::RefCell;
use std::collections::HashMap;


use std::rc::Rc;

pub struct Interpreter {
    pub context: Rc<RefCell<Context>>, // initally the root context, but this is a kinda tree like structure.
    pub builtin: HashMap<String, StandardFunction>,
    pub type_checker: TypeChecker,
}

impl Interpreter {
    pub fn visit_conditional_repeat_stmnt(
        &mut self,
        condition: &Option<Box<Node>>,
        block: &Box<Node>,
    ) -> Value {
        self.push_ctx();
        loop {
            let condition_result = match condition.as_ref() {
                Some(expression) => {
                    if let Value::Bool(val) = self.eval_deref(expression) {
                        val
                    } else {
                        dbg!(expression);
                        panic!("Expected boolean condition");
                    }
                }
                None => panic!("Expected condition in conditional repeat statement"),
            };
            if condition_result {
                self.push_ctx();
                let result = block.accept(self);
                self.pop_ctx();
                match &result {
                    Value::Int(..)
                    | Value::Double(..)
                    | Value::Bool(_)
                    | Value::Function(_)
                    | Value::String(_) => return result,
                    Value::Return(value) => {
                        if let Some(val) = value.clone() {
                            return *val;
                        } else {
                            return result;
                        }
                    }
                    _ => {}
                }
            } else {
                self.pop_ctx();
                return Value::None();
            }
        }
    }
    // this will seek parent contexts if & when the variable is not found in the current context.
    // this cannot be used to add new variables to a context.
    pub fn assign_var<'ctx>(&mut self, id: &str, value: &'ctx Value) {
        let Ok(_) = self
            .context
            .borrow_mut()
            .seek_overwrite_in_parents(&id, &value)
        else {
            panic!("assignment error : {} not found.", id);
        };
    }
    pub fn visit_conditionless_repeat_stmnt(&mut self, block: &Box<Node>) -> Value {
        loop {
            self.push_ctx();
            let _result = block.accept(self);
            self.pop_ctx();
            match _result {
                Value::Return(value) => {
                    if let Some(val) = value {
                        return *val;
                    } else {
                        return Value::None();
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }
    pub fn bin_op_float(&mut self, node: &Node, lhs: &f64, rhs: &f64) -> Value {
        let result: f64;

        let Node::BinaryOperation { lhs: _, op, rhs: _ } = node else {
            dbg!(node);
            panic!("Expected binary operation node");
        };

        match op {
            TokenKind::Add => result = lhs + rhs,
            TokenKind::Subtract => result = lhs - rhs,
            TokenKind::Multiply => result = lhs * rhs,
            TokenKind::Divide => result = lhs / rhs,
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }
        Value::Double(result)
    }
    pub fn bin_op_int(&mut self, node: &Node, lhs: &i32, rhs: &i32) -> Value {
        let result: i32;
        let Node::BinaryOperation { lhs: _, op, rhs: _ } = node else {
            dbg!(node);
            panic!("Expected binary operation node");
        };

        match op {
            TokenKind::Add => result = lhs + rhs,
            TokenKind::Subtract => result = lhs - rhs,
            TokenKind::Multiply => result = lhs * rhs,
            TokenKind::Divide => result = lhs / rhs,
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }
        Value::Int(result)
    }
    pub fn bin_op_string(&mut self, node: &Node, lhs: &String, rhs: &String) -> Value {
        let Node::BinaryOperation { lhs: _, op, rhs: _ } = node else {
            dbg!(node);
            panic!("Expected binary operation node");
        };

        let result: String;
        match op {
            TokenKind::Add => result = format!("{}{}", lhs, rhs),
            _ => {
                dbg!(node);
                panic!("invalid binary operation on strings");
            }
        }
        Value::String(result)
    }
    pub fn new() -> Interpreter {
        let builtins = super::standard_functions::get_builtin_functions();
        let type_checker = TypeChecker::new();
        let variables = HashMap::from([
            ("none".to_string(), Rc::new(RefCell::new(Instance{
                mutable: false,
                value: Value::None(),
                m_type: type_checker.get(NONE_TNAME).unwrap()
            })))
        ]);
        Interpreter {
            context: Rc::new(RefCell::new(Context {
                parent: None,
                variables
            })),
            builtin: builtins,
            type_checker: TypeChecker::new(),
        }
    }
    pub fn try_find_and_execute_fn(&mut self, arguments: &Value, id: &str) -> Value {
        
        let args = match arguments {
            Value::Tuple(values) => values,
            _ => panic!("Expected Tuple node"),
        };
        
        let function: Option<Rc<Function>>;
        {
            let mut ctx = self.context.borrow_mut();
            // function pointer
            let Some(fn_ptr) = ctx.find_variable(id) else {
                let Some(builtin) = self.builtin.get_mut(id) else {
                    dbg!(id);
                    panic!("Function {id}  not found");
                };
                return builtin.call(&mut ctx, &mut self.type_checker, args.clone());
            };
            
            function = match &fn_ptr.borrow_mut().value {
                Value::Function(func) => Some(func.clone()),
                _ => panic!("Expected function"),
            };
        }
        
        let Some(function) = function else {
            dbg!(id);
            panic!("Function {id} not found");
        };
        
        // valid parameterless
        if function.params.len() + args.len() == 0 {
            
            let result = function.body.accept(self);
            
            match result {
                Value::Return(Some(return_value)) => return *return_value,
                Value::Return(None) => return Value::None(),
                _ => return result,
            }
            
        }
        
        if args.len() != function.params.len() {
            panic!("Number of arguments does not match the number of parameters :: expected {}, got {}", function.params.len(), args.len());
        }
        
        self.push_ctx();
        
        for (arg, param) in args.iter().zip(function.params.iter()) {
            if !param.m_type.borrow().validate(arg) {
                panic!("Argument type does not match parameter type.\n provided argument: {:?} expected parameter : {:?}", arg, param)
            } else {
                self.context.borrow_mut().insert_variable(
                    &param.name,
                    Rc::new(RefCell::new(Instance::new(
                        false,
                        arg.clone(),
                        Rc::clone(&param.m_type),
                    ))),
                );
            }
        }
        
        let ret = function.body.accept(self);

        self.pop_ctx();

        if let Value::Return(Some(return_value)) = ret {
            return *return_value;
        }

        Value::None()
    }
    pub fn dot_op(&mut self, _lhs: &Box<Node>, _rhs: &Box<Node>) -> Value {
        Value::None()
    }
    pub fn push_ctx(&mut self) {
        let current = self.context.clone();

        self.context = Context::new();

        self.context.borrow_mut().parent = Some(Rc::clone(&current));
    }
    pub fn pop_ctx(&mut self) {
        let current = self.context.clone();
        
        self.context = match current.borrow_mut().parent.take() {
            Some(parent) => Rc::clone(&parent),
            None => panic!("Cannot pop root context"),
        };
    }
    pub fn access_array(&self, id: &str, index: usize) -> Value {
        let ctx = self.context.borrow();
        let var = ctx.find_variable(id).expect("Variable not found");
        let var = var.borrow();

        match &var.value {
            Value::Array(_, elements) => {
                let elements = elements.borrow();
                if elements.len() <= index {
                    panic!("Array index out of bounds :: {}[{}]", id, index);
                }
                elements[index].value.clone()
            }
            _ => panic!("Expected Array node"),
        }
    }
    pub fn assign_to_array(&mut self, id: &str, index: usize, value: Value) {
        let ctx = self.context.borrow();
        let var = ctx.find_variable(id).expect("Variable not found");
        let mut var = var.borrow_mut();

        match &mut var.value {
            Value::Array(mutable, elements) => {
                if !*mutable {
                    panic!("Cannot mutate immutable array");
                }
                let mut elements = elements.borrow_mut();
                if elements.len() <= index {
                    panic!("Array index out of bounds :: {}[{}]", id, index);
                }
                elements[index].set_value(&value);

                if !TypeChecker::validate(&elements[index]) {
                    dbg!(&elements[index]);
                    panic!("Invalid type");
                }
            }
            _ => panic!("Expected Array node"),
        }
    }
    pub fn evaluate_expression(&mut self, lhs: &Box<Node>, rhs: &Box<Node>, op: &TokenKind) -> Value {
        let lhs_value = self.eval_deref(lhs);
        let rhs_value = self.eval_deref(rhs);
        
        let l_type = self.type_checker.from_value(&lhs_value);
        let r_type = self.type_checker.from_value(&rhs_value);
        
        let l_type = match l_type {
            Some(t) => Rc::clone(&t),
            None => panic!("invalid type in relational expression : {:?}", lhs_value),
        };
        let r_type = match r_type {
            Some(t) => t,
            None => panic!("invalid type in relational expression : {:?}", rhs_value),
        };
        
        // function call
        if *op == TokenKind::OpenParenthesis {
            let func_id = get_identifier_str(&lhs);
            return self.try_find_and_execute_fn(&rhs_value, func_id);
        }
        
        let result = l_type.borrow().perform_bin_op(op, &r_type, &lhs_value, &rhs_value);
        result
    }
    pub fn eval_deref(&mut self, expression: &Node) -> Value {
        let value = expression.accept(self);
                
        let result = match &value {
            Value::Reference(inner) => {
                let inner = inner.borrow();
                inner.value.clone()
            }
            _ => value,
        };
        result
    }
}
impl Visitor<Value> for Interpreter {
    // top level nodes
    fn visit_program(&mut self, node: &Node) -> Value {
        let statements = match node {
            Node::Program(statements) => statements,
            _ => panic!("expected program node"),
        };

        for stmnt in statements {
            let result = stmnt.accept(self);
            match result {
                Value::Return(Some(return_value)) => return *return_value,
                Value::Return(None) => return Value::None(),
                _ => continue,
            }
        }

        Value::None()
    }
    fn visit_block(&mut self, node: &Node) -> Value {
        let statements = match node {
            Node::Block(statements) => statements,
            _ => panic!("Expected Block node"),
        };
        
        for statement in statements {
            let value = statement.accept(self);
            match value {
                Value::Return(..) => return value,
                _ => continue,
            }
        }

        Value::None()
    }
    // statements
    fn visit_if_stmnt(&mut self, node: &Node) -> Value {
        let (condition, true_block, else_block) = match node {
            Node::IfStmnt {
                condition,
                block: true_block,
                else_stmnt: else_block,
            } => (condition, true_block, else_block),
            _ => panic!("Expected WhereStmnt node"),
        };

        let condition_result = match condition.accept(self) {
            Value::Bool(condition_result) => condition_result,
            _ => panic!("Expected boolean condition"),
        };

        if condition_result {
            self.push_ctx();
            let returned = true_block.accept(self);
            self.pop_ctx();
            match returned {
                Value::Return(_) => return returned,
                _ => {}
            }
        } else if let Some(else_stmnt) = else_block {
            let returned = else_stmnt.accept(self);
            match returned {
                Value::Return(_) => return returned,
                _ => {}
            }
        }

        Value::None()
    }
    fn visit_else_stmnt(&mut self, node: &Node) -> Value {
        let (condition, true_block, else_stmnt) = match node {
            Node::ElseStmnt {
                condition,
                block: true_block,
                else_stmnt,
            } => (condition, true_block, else_stmnt),
            _ => panic!("Expected OrStmnt node"),
        };

        let condition_result = match condition.as_ref() {
            Some(expression) => match expression.accept(self) {
                Value::Bool(val) => val,
                _ => panic!("Expected boolean condition"),
            },
            None => true,
        };

        if condition_result {
            self.push_ctx();
            let returned = true_block.accept(self);
            self.pop_ctx();

            match returned {
                Value::Return(_) => return returned,
                _ => {}
            }

        } else if let Some(else_statement) = else_stmnt {
            else_statement.accept(self);
        }

        Value::None()
    }
    fn visit_declaration(&mut self, node: &Node) -> Value {
        if let Node::DeclStmt {
            target_id,
            target_type,
            expression,
            mutable,
        } = node
        {
            let value: Value;
            let var: Instance;
            let mutability = *mutable;
            
            let target_type = get_identifier_str(target_type);
            let target_id = get_identifier_str(target_id);
            
            match self.type_checker.get(target_type) {
                Some(m_type) => {
                    if expression.is_some() {
                        let expression = expression.as_ref().unwrap();
                        value = self.eval_deref(expression);
                    } else {
                        value = Value::None();
                    }
                    
                    var = Instance::new(mutability, value.clone(), m_type);
                    match value {
                        Value::None() => {
                            // all types can currently be none or something.
                        },
                        _ => {
                            if !TypeChecker::validate(&var) {
                                println!("recieved value: ");
                                dbg!(&var.value);
                                panic!("invalid type in declaration '{:?} : {}'",target_id, var.m_type.borrow().name);
                            }
                        }    
                    }
                }
                _ => {
                    dbg!(node);
                    panic!("{:?} is not a supported or known type.", target_type);
                }
            }
            {
                let mut ctx = self.context.borrow_mut();
                match ctx.find_variable(target_id) {
                    Some(_) => {
                        dbg!(node);
                        panic!("redefinition of variable {:?}", target_id);
                    }
                    None => {
                        ctx.insert_variable(&target_id, Rc::new(RefCell::new(var)));
                    }
                }
            }
        } else {
            panic!("Expected Declaration node");
        }
        Value::None()
    }
    fn visit_assignment(&mut self, node: &Node) -> Value {
        match node {
            Node::AssignStmnt { id, expression } => {
                
                let id = id.accept(self);
                
                let Value::Reference(id_val) = id else {
                    panic!("Expected Reference {:?}", id);
                };
                
                let result = self.eval_deref(expression);
                
                let mut id_val = id_val.borrow_mut();
                
                if !id_val.mutable {
					panic!("cannot assign to const : {:?}", id_val);
				}
                
                id_val.set_value(&result);
                
                if !TypeChecker::validate(&id_val) {
                    panic!("Invalid type {}", id_val.m_type.borrow().name);
                }
                
                return Value::None();
            }
            _ => {
                dbg!(node);
                panic!("Expected Assignment node");
            }
        }
    }

    // values
    fn visit_identifier(&mut self, node: &Node) -> Value {
        let ctx = self.context.borrow_mut();
        
        let Node::Identifier(id) = node else {
            dbg!(node);
            panic!("Expected Identifier");
        };
        
        let var = ctx.find_variable(id);
        
        let Some(var) = var else {
            dbg!(node);
            panic!("Variable {id} not found");
        };
        
        Value::Reference(Rc::clone(&var))
    }
    fn visit_bool(&mut self, node: &Node) -> Value {
        if let Node::Bool(value) = node {
            return Value::Bool(*value);
        } else {
            panic!("Expected Bool node");
        }
    }
    fn visit_number(&mut self, node: &Node) -> Value {
        if let Node::Double(value) = node {
            Value::Double(*value)
        } else if let Node::Int(value) = node {
            Value::Int(*value)
        } else {
            dbg!(node);
            panic!("Expected Number");
        }
    }
    fn visit_string(&mut self, node: &Node) -> Value {
        if let Node::String(value) = node {
            return Value::String(value.clone());
        } else {
            panic!("Expected String node");
        }
    }
    fn visit_eof(&mut self, _node: &Node) -> Value {
        Value::None() // do nothing.
    }
    
    // binary operations & expressions
    fn visit_relational_op(&mut self, node: &Node) -> Value {
        if let Node::RelationalOperation { lhs, op, rhs } = node {
            
            let lhs_value = self.eval_deref(lhs);
            let rhs_value = self.eval_deref(rhs);
            
            match (lhs_value, rhs_value) {
                (Value::Bool(lhs_bool), Value::Bool(rhs_bool)) => match op {
                    TokenKind::Equals => return Value::Bool(lhs_bool == rhs_bool),
                    TokenKind::NotEquals => return Value::Bool(lhs_bool != rhs_bool),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                (Value::Int(lhs_int), Value::Int(lhs_table)) => match op {
                    TokenKind::LeftAngle => return Value::Bool(lhs_int < lhs_table),
                    TokenKind::LessThanEquals => return Value::Bool(lhs_int <= lhs_table),
                    TokenKind::RightAngle => return Value::Bool(lhs_int > lhs_table),
                    TokenKind::GreaterThanEquals => return Value::Bool(lhs_int >= lhs_table),
                    TokenKind::Equals => return Value::Bool(lhs_int == lhs_table),
                    TokenKind::NotEquals => return Value::Bool(lhs_int != lhs_table),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                (Value::Int(lhs_float), Value::Double(rhs_float)) => match op {
                    TokenKind::LeftAngle => return Value::Bool((lhs_float as f64) < rhs_float),
                    TokenKind::LessThanEquals => {
                        return Value::Bool((lhs_float as f64) <= rhs_float)
                    }
                    TokenKind::RightAngle => return Value::Bool((lhs_float as f64) > rhs_float),
                    TokenKind::GreaterThanEquals => {
                        return Value::Bool((lhs_float as f64) >= rhs_float)
                    }
                    TokenKind::Equals => return Value::Bool((lhs_float as f64) == rhs_float),
                    TokenKind::NotEquals => return Value::Bool((lhs_float as f64) != rhs_float),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                (Value::Double(lhs_float), Value::Int(rhs_float)) => match op {
                    TokenKind::LeftAngle => return Value::Bool(lhs_float < (rhs_float as f64)),
                    TokenKind::LessThanEquals => {
                        return Value::Bool(lhs_float <= (rhs_float as f64))
                    }
                    TokenKind::RightAngle => return Value::Bool(lhs_float > (rhs_float as f64)),
                    TokenKind::GreaterThanEquals => {
                        return Value::Bool(lhs_float >= (rhs_float as f64))
                    }
                    TokenKind::Equals => return Value::Bool(lhs_float == (rhs_float as f64)),
                    TokenKind::NotEquals => return Value::Bool(lhs_float != (rhs_float as f64)),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                (Value::Double(lhs_float), Value::Double(rhs_float)) => match op {
                    TokenKind::LeftAngle => return Value::Bool(lhs_float < rhs_float),
                    TokenKind::LessThanEquals => return Value::Bool(lhs_float <= rhs_float),
                    TokenKind::RightAngle => return Value::Bool(lhs_float > rhs_float),
                    TokenKind::GreaterThanEquals => return Value::Bool(lhs_float >= rhs_float),
                    TokenKind::Equals => return Value::Bool(lhs_float == rhs_float),
                    TokenKind::NotEquals => return Value::Bool(lhs_float != rhs_float),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                (Value::String(lhs_string), Value::String(rhs_string)) => match op {
                    TokenKind::Equals => return Value::Bool(lhs_string == rhs_string),
                    TokenKind::NotEquals => return Value::Bool(lhs_string != rhs_string),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                _ => {
                    self.evaluate_expression(lhs, rhs, op)
                }   
            }
        } else {
            panic!("Expected RelativeExpression node");
        }
    }
    fn visit_logical_op(&mut self, node: &Node) -> Value {
        if let Node::LogicalOperation { lhs, op, rhs } = node {
            let lhs_value = self.eval_deref(lhs);
            let rhs_value = self.eval_deref(rhs);
            match (lhs_value, rhs_value) {
                (Value::Bool(lhs_bool), Value::Bool(rhs_bool)) => match op {
                    TokenKind::LogicalAnd => return Value::Bool(lhs_bool && rhs_bool),
                    TokenKind::LogicalOr => return Value::Bool(lhs_bool || rhs_bool),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator for logical / boolean expression");
                    }
                },
                _ => {
                    self.evaluate_expression(lhs, rhs, op)
                }
            }
        } else {
            panic!("Expected LogicalExpression node");
        }
    }
    fn visit_expression(&mut self, node: &Node) -> Value {
        if let Node::Expression(root) = node {
            return root.accept(self);
        } else {
            panic!("Expected Expression node");
        }
    }
    fn visit_binary_op(&mut self, node: &Node) -> Value {
        let Node::BinaryOperation { lhs, op, rhs } = node else {
            dbg!(node);
            panic!("Expected binary operation node");
        };
        match op {
            TokenKind::Dot => self.dot_op(lhs, rhs),
            TokenKind::Add | TokenKind::Divide | TokenKind::Multiply | TokenKind::Subtract => {
                let e_lhs = self.eval_deref(lhs);
                let e_rhs = self.eval_deref(rhs);
                match (e_lhs, e_rhs) {
                    (Value::Int(lhs_float), Value::Int(rhs_float)) => self.bin_op_int(node, &lhs_float, &rhs_float),
                    (Value::Double(lhs_float), Value::Double(rhs_float)) => self.bin_op_float(node, &lhs_float, &rhs_float),
                    (Value::String(lhs_string), Value::String(rhs_string)) => self.bin_op_string(node, &lhs_string, &rhs_string),
                    _ => self.evaluate_expression(lhs, rhs, op)
                }
            }
            _ => self.evaluate_expression(lhs, rhs, op),
        }
    }
    fn visit_term_op(&mut self, node: &Node) -> Value {
        if let Node::BinaryOperation { lhs, op, rhs } = node {
            let lhs_value = self.eval_deref(lhs);
            let rhs_value = self.eval_deref(rhs);
            match (lhs_value, rhs_value) {
                (Value::Int(lhs_int), Value::Int(rhs_int)) => match op {
                    TokenKind::Multiply => return Value::Int(lhs_int * rhs_int),
                    TokenKind::Divide => return Value::Int(lhs_int / rhs_int),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator for integer expression");
                    }
                },
                (Value::Double(lhs_float), Value::Double(rhs_float)) => match op {
                    TokenKind::Multiply => return Value::Double(lhs_float * rhs_float),
                    TokenKind::Divide => return Value::Double(lhs_float / rhs_float),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator for float expression");
                    }
                },
                _ => {
                    self.evaluate_expression(lhs, rhs, op)
                }
            }
        } else {
            panic!("Expected BinaryOperation node");
        }
    }
 
    fn visit_function_decl(&mut self, node: &Node) -> Value {
        if let Node::FuncDeclStmnt {
            id,
            params: fn_decl_params,
            body,
            return_t,
            mutable,
        } = node
        {
            
            
            let return_type = get_identifier_str(return_t);
            let id = get_identifier_str(id);
            
            let body_cloned = body.clone();
            let Some(r_type) = self.type_checker.get(return_type) else {
                panic!("FnDecl: {} not a valid return type", return_type);
            };
            
            let parameters = Vec::new();
            
            let tuple = fn_decl_params.accept(self);
        
            let Value::KeyTypeTuple(_values) = tuple else {
                panic!("Expected KeyValueTuple node");
            }; 
         
            let func = Function {
                name: id.to_string(),
                params: parameters,
                body: body_cloned,
                return_type: r_type,
                mutable: *mutable,
            };
            // Todo: we might want to have a better way to do this than just getting it by string
            let Some(m_type) = self.type_checker.get(FN_TNAME) else {
                panic!("Fn isn't a type");
            };
            let function = Instance {
                mutable: *mutable,
                value: Value::Function(Rc::new(func)),
                m_type,
            };
            self.context
                .borrow_mut()
                .insert_variable(&id, Rc::new(RefCell::new(function)));
        } else {
            panic!("Expected FunctionDecl node");
        };
        Value::None()
    }
    
    fn visit_while_stmnt(&mut self, node: &Node) -> Value {
        let Node::WhileStmnt {
            condition,
            block,
        } = node
        else {
            dbg!(node);
            panic!("Expected WhileStmnt node");
        };
        
        
        if condition.is_none() {
            self.visit_conditionless_repeat_stmnt(block)
        } else {
            self.visit_conditional_repeat_stmnt(condition, block)
        }
        
    }
    fn visit_return_stmnt(&mut self, node: &Node) -> Value {
        if let Node::ReturnStmnt(opt_val) = node {
            let Some(value_node) = opt_val else {
                return Value::Return(None);
            };
            let value = value_node.accept(self);
            return Value::Return(Some(Box::new(value.clone())));
        } else {
            panic!("Expected ReturnStmnt node");
        }
    }
    fn visit_array(&mut self, node: &Node) -> Value {
        if let Node::Array {
            typename: _,
            init_capacity,
            elements,
            mutable: mutability,
            elements_mutable,
        } = node
        {
            let len = *init_capacity;
            if len < elements.len() {
                panic!("Array length is less than the number of elements");
            }
            let mut values = Vec::with_capacity(len);
            for value in elements {
                let val = value.accept(self);
                // TODO curently not checking if type is valid
                let Some(m_type) = self.type_checker.from_value(&val) else {
                    dbg!(&node);
                    panic!("{:?} doesn't match to a valid type", val);
                };
                let var = Instance::new(*elements_mutable, val, m_type);
                values.push(var);
            }

            return Value::Array(*mutability, Rc::new(RefCell::new(values)));
        } else {
            panic!("Expected List node");
        }
    }
   
   
    fn visit_struct_def(&mut self, node: &Node) -> Value {
        if let Node::StructDecl { id, block } = node {
            let Node::Block(_statements) = block.as_ref() else {
                panic!("Expected block")
            };
            
            let struct_tname = get_identifier_str(id);
            
            self.push_ctx();
            
            let mut fields = Vec::<(String, Rc<RefCell<Type>>)>::new();
            
            for statement in _statements {
                let Node::DeclStmt {
                    target_type, target_id, ..
                } = statement.as_ref()
                else {
                    panic!("Expected declaration, got {:#?}", statement);
                };
                let target_type = get_identifier_str(target_type);
                let target_id = get_identifier_str(target_id);
                
                let Some(t) = self.type_checker.get(&target_type) else {
                    panic!("{} not a valid type", target_type);
                };
                
                fields.push((target_id.to_string(), t));
                statement.accept(self);
            }
            
            let _new_type = Type {
                name: struct_tname.to_string(),
                validator: Box::new(|_value| true),
                attribute: Attr::Struct,
                // todo : make sure this clones fully and doesn't just copy the reference.
                context: Box::new(self.context.borrow().to_owned()),
                operators: Vec::new(),
            };
            
            self.type_checker.types.insert(struct_tname.to_string(), Rc::new(RefCell::new(_new_type)));
            
            self.pop_ctx();
        }
        Value::None()
    }

    fn visit_type_assoc_block(&mut self, node: &Node) -> Value {
        let Node::TypeAssocBlockStmnt { typename, block } = node else {
            panic!("Expected TypeAssocBlock node");
        };
        
        let typename = get_identifier_str(&typename);
        let type_ = if let Some(struct_) = self.type_checker.types.get_mut(&typename.to_string()) {
            struct_
        } else {
            panic!("Struct {} not found", typename);
        };
        
        // clone boxed context
        let mut type_context = type_.borrow_mut().context.clone();
        
        type_context.parent = Some(Rc::clone(&self.context));
        
        // make rc to ctx
        let rc = Rc::new(RefCell::new(*type_context));
        
        self.context = Rc::clone(&rc);
        
        let ctx = Some(Rc::clone(&rc));
        
        block.accept(self);
        
        if let Some(ctx) = ctx {
            if let Some(_struct) = self.type_checker.get(&typename) {
                _struct.borrow_mut().context = Box::new(ctx.borrow().to_owned());
            }
        }
        
        self.pop_ctx();
        
        Value::None()
    }
    
    fn visit_tuple(&mut self, node: &Node) -> Value {
        let Node::Tuple(values) = node else {
            panic!("Expected Tuple node");
        };
        let mut result = Vec::new();
        for expr in values {
            let val = expr.accept(self);
            result.push(val);
        }
        Value::Tuple(result)
    }
    
    fn visit_unary_op(&mut self, node: &Node) -> Value {
        match &node {
            Node::NotOp(value) => {
                match value.accept(self) {
                    Value::Bool(value) => Value::Bool(!value),
                    _ => panic!("Expected boolean operand for unary not (!) operation"),
                }
            }
            Node::NegOp(operand) => {
                match operand.accept(self) {
                    Value::Double(value) => Value::Double(-value),
                    Value::Int(value) => Value::Int(-value),
                    _ => panic!("Expected numeric operand for unary negation (-) operation"),
                }
            }
            _ => {
                dbg!(node);
                panic!("Expected unary operation node");
            }
        }
    }
    
    fn visit_kv_tuple(&mut self, _node: &Node) -> Value {
        let Node::KeyValueTuple { pairs } =_node else {
            panic!("Expected KeyValueTuple node");
        };
        let mut v_pairs = Vec::new();
        for pair in pairs {
            let kvp = pair.accept(self);
            v_pairs.push(kvp);
        }
        Value::KeyTypeTuple(v_pairs)
    }
    
    fn visit_kv_pair(&mut self, node: &Node) -> Value {
        let Node::KeyValuePair { varname, typename } = node else {
            panic!("Expected KeyValuePair node");
        };
        let varname = get_identifier_str(varname);
        let typename = get_identifier_str(typename);
        
        let t = self.type_checker.get(typename);
        
        if t.is_none() {
            panic!("{} is not a valid type", typename);
        }
        
        let t = t.unwrap();
        
        return Value::KeyTypePair(varname.to_string(), t)
    }
}

// todo : improve this, right now we don't have type paths but we will.
// also, this never accounts for bin ops like dot op or :: 
fn get_identifier_str(target_type: &Node) -> &str {
    if let Node::Identifier(id) = target_type {
        return id;
    } else {
        panic!("Expected Identifier node");
    }
}
