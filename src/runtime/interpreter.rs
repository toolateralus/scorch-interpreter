use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::frontend::ast::*;
use crate::frontend::tokens::*;
use super::context::Context;
use super::std_builtins::BuiltInFunction;
use super::typechecker::*;
use super::types::*;

pub struct Interpreter {
    pub context: Rc<RefCell<Context>>, // initally the root context, but this is a kinda tree like structure.
    pub builtin: HashMap<String, BuiltInFunction>,
    pub type_checker: TypeChecker,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let builtins = super::std_builtins::get_builtin_functions();
        Interpreter {
            context: Context::new(),
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

        if function.params.len() + args.len() == 0 {
            return function.body.accept(self);
        }

        if args.len() != function.params.len() {
            panic!("Number of arguments does not match the number of parameters");
        }

        self.push_ctx();

        for (arg, param) in args.iter().zip(function.params.iter()) {
            if !param.m_type.validate(arg) {
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
        {
            let ctx = self.context.borrow_mut();
            if let Node::Identifier(id) = lhs.as_ref() {
                let tempvar = ctx.find_variable(id);
                let Some(var) = tempvar else {
                    dbg!(lhs, rhs);
                    panic!("Expected Struct node");
                };
                let Value::Struct {
                    typename: _,
                    context,
                } = &var.borrow_mut().value
                else {
                    dbg!(lhs, rhs);
                    panic!("Expected Struct node");
                };
                let Node::Identifier(id) = rhs.as_ref() else {
                    dbg!(lhs, rhs);
                    panic!("Expected Struct node");
                };

                let Some(var) = context.find_variable(id) else {
                    dbg!(lhs, rhs);
                    panic!("Expected Struct node");
                };

                return var.borrow_mut().value.clone();
            };
        }

        let Node::FunctionCall {
            id: func_id,
            arguments,
        } = rhs.as_ref()
        else {
            dbg!(lhs, rhs);
            panic!("Expected FunctionCall node");
        };

        let Some(args) = arguments else {
            dbg!(lhs, rhs);
            panic!("Expected arguments");
        };

        let mut args = args.clone();
        args.insert(0, *lhs.clone());

        self.try_find_and_execute_fn(&Some(args), func_id)
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
                Value::Return(Some(return_value)) => return *return_value,
                Value::Return(None) => return Value::Return(None),
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
            match returned {
                Value::Return(_) => return returned,
                _ => {}
            }
            self.pop_ctx();
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

            match returned {
                Value::Return(_) => return returned,
                _ => {}
            }

            self.pop_ctx();
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
                        dbg!(&var);
                        panic!("invalid type");
                    }
                }
                _ => {
                    dbg!(node);
                    panic!("Unsupported type");
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

    // literals & values
    // todo: move this into it's own visitor, previous to this one? it needs a
    // different return type otherwise reference counting nad pointers will be very very challenging, as far as i can see.
    // there's probably a way.
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
                panic!("Variable not found");
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
                    dbg!(node);
                    panic!("mismatched type in relative expression");
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
                        panic!("invalid operator");
                    }
                },
                _ => {
                    dbg!(node);
                    panic!("mismatched type in logical expression");
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
                    (Value::Int(lhs_float), Value::Int(rhs_float)) => {
                        return self.bin_op_int(node, &lhs_float, &rhs_float);
                    }
                    (Value::Double(lhs_float), Value::Double(rhs_float)) => {
                        return self.bin_op_float(node, &lhs_float, &rhs_float);
                    }
                    (Value::String(lhs_string), Value::String(rhs_string)) => {
                        return self.bin_op_string(node, &lhs_string, &rhs_string);
                    }
                    _ => {
                        dbg!(node);
                        panic!("mismatched type in binary operation");
                    }
                }
            }
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
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

    // functions
    fn visit_param_decl(&mut self, _node: &Node) -> Value {
        todo!()
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
            let Some(m_type) = self.type_checker.get("Fn") else {
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
        
        let mut var: Option<Rc<RefCell<Instance>>> = None;
        
        {
            let ctx = self.context.borrow_mut();
            var = ctx.find_variable(id);
        }
        
        let Some(var) = var else {
            panic!("Variable {:?} not found", id);
        };

        let var = var.borrow_mut();

        let (mutable, elements) = match &var.value {
            Value::Array(mutable, elements) => (mutable, elements),
            _ => panic!("Expected Array node"),
        };

        let index_value = match index.accept(self) {
            Value::Double(index_value) => index_value as usize,
            Value::Int(index_value) => index_value as usize,
            _ => panic!("Expected numerical index value"),
        };

        let mut elements = elements.borrow_mut();

        if elements.len() <= index_value {
            panic!("Array index out of bounds :: {}[{}]", id, index_value);
        }

        let element = &mut elements[index_value];

        // read
        if !*assignment {
            return element.value.clone();
        }
        
        if !*mutable {
            panic!("Cannot mutate immutable array");
        }
        
        // assignment
        if let Some(expr) = expression {
            let result = expr.accept(self);
            
            element.set_value(&result);
            
            if !TypeChecker::validate(&element) {
                dbg!(&element);
                panic!("Invalid type");
            }
            
            let mut ctx = self.context.borrow_mut();
            
            let Ok(_) = ctx.seek_overwrite_in_parents(id, &element.value) else {
                panic!("Variable {:?} not found", id);
            };

            return Value::None();
        }

        panic!("Expected expression in array assignment");
    }
    fn visit_lambda(&mut self, _node: &Node) -> Value {
        todo!()
    }
    fn visit_struct_def(&mut self, node: &Node) -> Value {
        if let Node::StructDecl { id, block } = node {
            let Node::Block(_statements) = block.as_ref() else {
                panic!("Expected block")
            };

            let _new_type = Type {
                name: id.to_string(),
                validator: Box::new(|_value| true),
                attribute: Attr::Struct,
            };

            let mut fields = Vec::<(String, Rc<Type>)>::new();

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
            }

            let _struct = Struct {
                name: id.to_string(),
                fields, // todo : pre-evaluate default values for fields and just copy that context into new structs instead of what we do now.
                type_: Rc::new(_new_type),
            };

            self.type_checker
                .structs
                .insert(id.to_string(), Box::new(_struct));
        }
        Value::None()
    }
    fn visit_struct_init(&mut self, node: &Node) -> Value {
        let Node::Struct { id, args } = node else {
            panic!("Expected StructInit node");
        };

        let mut struct_ctx = Context {
            parent: Some(Rc::clone(&self.context)),
            variables: HashMap::new(),
        };

        let _struct = if let Some(_struct) = self.type_checker.structs.get_mut(id) {
            _struct
        } else {
            panic!("Struct {} not found", id);
        };

        let fields = _struct.fields.clone();

        if fields.len() != args.len() {
            panic!("{id} constructor:  number of arguments does not match the number of fields");
        }

        for (field, arg) in fields.iter().zip(args.iter()) {
            let value = arg.accept(self);

            let Some(t) = self.type_checker.from_value(&value) else {
                panic!(
                    "type error: cannot find a type that matches value : {:#?}",
                    value
                );
            };

            let expected_typename = field.1.as_ref().name.clone();
            let found_typename = t.name.clone();
            if expected_typename != "Dynamic" && expected_typename != found_typename {
                panic!(
                    "type mismatch in '{id}' constructor. expected {:?}, got {:?}",
                    field.1.as_ref().name,
                    t.name
                );
            }

            let var = Instance::new(true, value, Rc::clone(&t));
            struct_ctx.insert_variable(&field.0, Rc::new(RefCell::new(var)));
        }

        Value::Struct {
            typename: id.clone(),
            context: Box::new(struct_ctx),
        }
    }
}
