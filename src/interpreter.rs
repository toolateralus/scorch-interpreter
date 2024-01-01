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
        id: &str,
        condition: &Option<Box<Node>>,
        block: &Box<Node>,
    ) -> Value {
        let mut iter: i32 = 0;

        let typename = INT_TNAME.to_string();

        self.push_ctx();

        {
            let mut ctx = self.context.borrow_mut();

            match ctx.find_variable(&id) {
                Some(v) => {
                    let v = v.borrow_mut();

                    if v.mutable == false {
                        panic!("Cannot mutate immutable variable {} in a repeat loop", id);
                    }
                }
                None => {
                    let val = Value::Int(0);
                    let Some(m_type) = self.type_checker.get(INT_TNAME) else {
                        panic!("Double isnt a type")
                    };

                    let var = Instance::new(true, val, m_type);
                    
                    if !TypeChecker::validate(&var) {
                        panic!(
                            "Invalid type for variable {} (generated by a 'repeat' loop)",
                            id
                        );
                    }

                    ctx.insert_variable(&id, Rc::new(RefCell::new(var)));
                }
            }
        }

        let Some(_m_type) = self.type_checker.get(typename.as_str()) else {
            panic!("{} isnt a type", typename)
        };

        loop {
            let condition_result = match condition.as_ref() {
                Some(expression) => {
                    if let Value::Bool(val) = expression.accept(self) {
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
                // pop iterator ctx.
                let value = Value::Int(iter);

                self.assign_var(id, &value);

                self.pop_ctx();
                return Value::None();
            }

            iter += 1;

            let value = Value::Int(iter);

            self.assign_var(id, &value);
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
    
    pub fn get_params_list(&mut self, param_nodes: &Vec<Node>) -> Vec<Parameter> {
        let mut params = Vec::new();
        for param in param_nodes {
            if let Node::ParamDeclNode { varname, typename } = param {
                let param_name = match varname.as_ref() {
                    Node::Identifier(id) => id.clone(),
                    _ => {
                        dbg!(varname);
                        panic!("Expected Identifier node");
                    }
                };

                let type_name = match typename.as_ref() {
                    Node::Identifier(id) => id.clone(),
                    _ => {
                        dbg!(typename);
                        panic!("Expected Identifier node");
                    }
                };

                let Some(m_type) = self.type_checker.get(type_name.as_str()) else {
                    panic!("{} isnt a type", type_name)
                };

                let parameter = Parameter {
                    name: param_name,
                    m_type,
                };

                params.push(parameter);
            }
        }
        params
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
    pub fn try_find_and_execute_fn(&mut self, arguments: &Option<Vec<Node>>, id: &String) -> Value {
        let args = Function::extract_args(self, arguments);
        
        let function: Option<Rc<Function>>;
        
        {
            let mut ctx = self.context.borrow_mut();
            // function pointer
            let Some(fn_ptr) = ctx.find_variable(id) else {
                let Some(builtin) = self.builtin.get_mut(id) else {
                    dbg!(id);
                    panic!("Function not found");
                };
                return builtin.call(&mut ctx, &self.type_checker, args);
            };
            
            function = match &fn_ptr.borrow_mut().value {
                Value::Function(func) => Some(func.clone()),
                _ => panic!("Expected function"),
            };
        }
        
        let Some(function) = function else {
            dbg!(id);
            panic!("Function not found");
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

    pub fn dot_op(&mut self, lhs: &Box<Node>, rhs: &Box<Node>) -> Value {
        let lhs_value = lhs.accept(self);
        
        match rhs.as_ref() {
            Node::Identifier(id) => match lhs_value {
                Value::StructInstance {
                    typename,
                    context,
                } => {
                    let Some(var) = context.find_variable(id) else {
                        panic!("unable to find variable {id} in struct {typename}");
                    };
                    return var.borrow().value.clone();
                }
                _ => {
                    panic!("expected struct");
                }
            },
            Node::FunctionCall { id, arguments } => {
                if let Some(mut args) = arguments.clone() {
                    args.insert(0, *lhs.clone());
                    self.try_find_and_execute_fn(&Some(args), id)
                } else {
                    let mut args = Vec::new();
                    args.push(*lhs.clone());
                    self.try_find_and_execute_fn(&Some(args), id)
                }
            }
            _ => {
                dbg!(lhs, rhs);
                panic!("Unexpected node type");
            }
        }
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

    fn evaluate_expression(&mut self, lhs: &Box<Node>, rhs: &Box<Node>, op: &TokenKind) -> Value {
        let lhs_value = lhs.accept(self);
        let rhs_value = rhs.accept(self);
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
        let result = l_type.borrow().perform_bin_op(op, &r_type, &lhs_value, &rhs_value);
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
            target_type,
            id,
            expression,
            mutable,
        } = node
        {
            let value: Value;
            let var: Instance;
            let mutability = *mutable;

            match self.type_checker.get(target_type.as_str()) {
                Some(m_type) => {
                    
                    value = expression.accept(self);
                    var = Instance::new(mutability, value, m_type);
                    
                    if !TypeChecker::validate(&var) {
                        println!("recieved value: ");
                        dbg!(&var.value);
                        panic!("invalid type in declaration '{id} : {}'", var.m_type.borrow().name);
                    }
                }
                _ => {
                    dbg!(node);
                    panic!("{} is not a supported or known type.", target_type);
                }
            }
            {
                let mut ctx = self.context.borrow_mut();
                match ctx.find_variable(&id) {
                    Some(_) => {
                        dbg!(node);
                        panic!("redefinition of variable {id}");
                    }
                    None => {
                        ctx.insert_variable(&id, Rc::new(RefCell::new(var)));
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
                let val: Value;
                val = self.visit_expression(expression);
                let str_id: String = match id.as_ref() {
                    Node::Identifier(id) => id.clone(),
                    _ => {
                        dbg!(node);
                        panic!("Expected Identifier node");
                    }
                };

                self.assign_var(&str_id, &val);

                return Value::None();
            }
            _ => {
                dbg!(node);
                panic!("Expected Assignment node");
            }
        }
    }
    
    fn visit_identifier(&mut self, node: &Node) -> Value {
        let ctx = self.context.borrow_mut();
        
        let Node::Identifier(id) = node else {
            dbg!(node);
            panic!("Expected Identifier");
        };
        
        match ctx.find_variable(id) {
            Some(value) => value.borrow_mut().value.clone(),
            None => {
                dbg!(node);
                panic!("variable {} not found", id);
            }
        }
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

    // unary operations
    fn visit_not_op(&mut self, node: &Node) -> Value {
        if let Node::NotOp(operand) = node {
            match operand.accept(self) {
                Value::Bool(value) => Value::Bool(!value),
                _ => panic!("Expected boolean operand for unary not (!) operation"),
            }
        } else {
            panic!("Expected NotOp node");
        }
    }
    fn visit_neg_op(&mut self, node: &Node) -> Value {
        if let Node::NegOp(operand) = node {
            match operand.accept(self) {
                Value::Double(value) => Value::Double(-value),
                Value::Int(value) => Value::Int(-value),
                _ => panic!("Expected numeric operand for unary negation (-) operation"),
            }
        } else {
            panic!("Expected NegOp node");
        }
    }

    // binary operations & expressions
    fn visit_relational_expression(&mut self, node: &Node) -> Value {
        if let Node::RelationalExpression { lhs, op, rhs } = node {
            let lhs_value = lhs.accept(self);
            let rhs_value = rhs.accept(self);
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
        
    fn visit_logical_expression(&mut self, node: &Node) -> Value {
        if let Node::LogicalExpression { lhs, op, rhs } = node {
            let lhs_value = lhs.accept(self);
            let rhs_value = rhs.accept(self);
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
                let e_lhs = lhs.accept(self);
                let e_rhs = rhs.accept(self);
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
    fn visit_term(&mut self, _node: &Node) -> Value {
        return Value::None();
    }
    fn visit_factor(&mut self, node: &Node) -> Value {
        match node {
            Node::Expression(root) => root.accept(self),
            _ => {
                dbg!(node);
                panic!("Expected Number or Identifier node");
            }
        }
    }
  
    fn visit_function_call(&mut self, node: &Node) -> Value {
        let (id, arguments) = match node {
            Node::FunctionCall { id, arguments } => (id, arguments),
            _ => return Value::None(),
        };
        self.try_find_and_execute_fn(arguments, id)
    }
    fn visit_function_decl(&mut self, node: &Node) -> Value {
        if let Node::FnDeclStmnt {
            id,
            params,
            body,
            return_type,
            mutable,
        } = node
        {
            let body_cloned = body.clone();
            let Some(r_type) = self.type_checker.get(return_type) else {
                panic!("FnDecl: {} not a valid return type", return_type);
            };
            let func = Function {
                name: id.to_string(),
                params: self.get_params_list(params),
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
    
    fn visit_repeat_stmnt(&mut self, node: &Node) -> Value {
        let Node::RepeatStmnt {
            iterator_id,
            condition,
            block,
        } = node
        else {
            dbg!(node);
            panic!("Expected RepeatStmnt node");
        };

        match iterator_id {
            // see expression for the implementation of these function
            // with a conditional expression
            Some(id) => self.visit_conditional_repeat_stmnt(id, condition, block),
            // without a conditional expression
            None => self.visit_conditionless_repeat_stmnt(block),
        }
    }
    fn visit_break_stmnt(&mut self, node: &Node) -> Value {
        if let Node::BreakStmnt(opt_val) = node {
            let Some(value_node) = opt_val else {
                return Value::Return(None);
            };
            let value = value_node.accept(self);
            return Value::Return(Some(Box::new(value.clone())));
        } else {
            panic!("Expected BreakStmnt node");
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
    fn visit_array_access(&mut self, node: &Node) -> Value {
        let (id, index, expression, assignment) = match node {
            Node::ArrayAccessExpr {
                id,
                index_expr: index,
                expression,
                assignment,
            } => (id, index, expression, assignment),
            _ => panic!("Expected ArrayAccessExpr node"),
        };

        let index_value = match index.accept(self) {
            Value::Double(index_value) => index_value as usize,
            Value::Int(index_value) => index_value as usize,
            _ => panic!("Expected numerical index value"),
        };

        if *assignment {
            if let Some(expr) = expression {
                let result = expr.accept(self);
                self.assign_to_array(id, index_value, result);
            } else {
                panic!("Expected expression in array assignment");
            }
            Value::None()
        } else {
            self.access_array(id, index_value)
        }
    }
   
    fn visit_struct_def(&mut self, node: &Node) -> Value {
        if let Node::StructDecl { id, block } = node {
            let Node::Block(_statements) = block.as_ref() else {
                panic!("Expected block")
            };
            
            self.push_ctx();
            
            let mut fields = Vec::<(String, Rc<RefCell<Type>>)>::new();
            
            for statement in _statements {
                let Node::DeclStmt {
                    target_type, id, ..
                } = statement.as_ref()
                else {
                    panic!("Expected declaration, got {:#?}", statement);
                };
                
                let Some(t) = self.type_checker.get(&target_type) else {
                    panic!("{} not a valid type", target_type);
                };
                
                fields.push((id.clone(), t));
                statement.accept(self);
            }
            
            let _new_type = Type {
                name: id.to_string(),
                validator: Box::new(|_value| true),
                attribute: Attr::Struct,
                // todo : make sure this clones fully and doesn't just copy the reference.
                context: Box::new(self.context.borrow().to_owned()),
                operators: Vec::new(),
            };
            
            self.type_checker.types.insert(id.to_string(), Rc::new(RefCell::new(_new_type)));
            
            self.pop_ctx();
        }
        Value::None()
    }
    fn visit_struct_init(&mut self, node: &Node) -> Value {
        let Node::StructInit { id, .. } = node else {
            panic!("Expected StructInit node");
        };
        
        let typedef = if let Some(type_) = self.type_checker.types.get_mut(id) {
            type_
        } else {
            panic!("Struct {} not found", id);
        };
        
        let struct_context = typedef.borrow().context.clone();
        
        Value::StructInstance {
            typename: id.clone(),
            context: struct_context,
        }
    }
    fn visit_type_assoc_block(&mut self, node: &Node) -> Value {
        let Node::TypeAssocBlock { typename, block } = node else {
            panic!("Expected TypeAssocBlock node");
        };
        
        let typename_clone = typename.clone();
        let _struct = if let Some(_struct) = self.type_checker.types.get_mut(&typename_clone) {
            _struct
        } else {
            panic!("Struct {} not found", typename);
        };
        
        // clone boxed context
        let mut struct_context = _struct.borrow().context.clone();
        
        struct_context.parent = Some(Rc::clone(&self.context));
        // make rc to ctx
        let rc = Rc::new(RefCell::new(*struct_context));
        
        self.context = Rc::clone(&rc);
        
        let ctx = Some(Rc::clone(&rc));
        
        block.accept(self);
        
        if let Some(ctx) = ctx {
            if let Some(_struct) = self.type_checker.get(&typename_clone) {
                _struct.borrow_mut().context = Box::new(ctx.borrow().to_owned());
            }
        }
        
        self.pop_ctx();
        
        Value::None()
    }
}
