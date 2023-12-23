use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::frontend::ast::*;
use crate::frontend::tokens::*;

use super::typechecker::*;
use super::types::*;

pub struct Interpreter {
    pub context: Context, // initally the root context, but this is a kinda tree like structure.
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
    
    fn try_find_and_execute_fn(
        &mut self,
        arguments: &Option<Vec<Node>>,
        id: &String,
    ) -> Value {
        let args = Function::extract_args(self, arguments);

        // builtin functions
        if self.builtin.contains_key(id) {
            let builtin = self.builtin.get_mut(id).unwrap();
            return builtin.call(args.clone());
        }
        
        // function pointer
        let Some(fn_ptr) = self.context.find_variable(id) else {
            panic!("Function not found");
        };
        let function = match fn_ptr.value.clone() {
            Value::Function(func) => func.clone(),
            _ => panic!("Expected function"),
        };

        if function.params.len() + args.len() == 0 {
            return function.body.accept(self);
        }

        if args.len() != function.params.len() {
            panic!("Number of arguments does not match the number of parameters");
        }

        for (arg, param) in args.iter().zip(function.params.iter()) {
            if !param.m_type.validate(arg) {
                panic!("Argument type does not match parameter type.\n provided argument: {:?} expected parameter : {:?}", arg, param)
            } else {
                self.context.insert_variable(
                    &param.name,
                    Rc::new(Variable::new(false, arg.clone(), Rc::clone(&param.m_type))),
                );
            }
        }

        let ret = function.body.accept(self);
        if let Value::Return(Some(return_value)) = ret {
            return *return_value;
        }

        Value::None()
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
            let stmnts = match &**true_block {
                Node::Block(stmnts) => stmnts,
                _ => panic!("Expected Block node"),
            };

            for stmnt in stmnts {
                let value = stmnt.accept(self);
                if let Value::Return(_) = value {
                    return value;
                }
            }
        } else if let Some(else_stmnt) = else_block {
            else_stmnt.accept(self);
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
            true_block.accept(self);
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
            let var: Variable;
            let mutability = *mutable;

            match self.type_checker.get(target_type.as_str()) {
                Some(m_type) => {
                    value = expression.accept(self);
                    var = Variable::new(mutability, value, m_type);
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

            match self.context.find_variable(&id) {
                Some(_) => {
                    dbg!(node);
                    panic!("redefinition of variable");
                }
                None => {
                    self.context.insert_variable(&id, Rc::new(var));
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
                match self.context.variables.get_mut(&str_id) {
                    Some(value) => {
                        if value.mutable == false {
                            dbg!(node);
                            panic!("Cannot assign to immutable variable");
                        }
                        
                        *value = Rc::new(Variable::new(
							value.mutable,
							val,
							Rc::clone(&value.m_type)
						));
                        if TypeChecker::validate(value) == false {
                            dbg!(node);
                            panic!("Type mismatch");
                        }
                    }
                    None => {
                        dbg!(node);
                        panic!("Variable not found");
                    }
                }
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
        let Node::Identifier(id) = node else {
            dbg!(node);
            panic!("Expected Identifier");
        };
        match self.context.find_variable(id) {
            Some(value) => value.value.clone(),
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
        match node {
            Node::DotOp { lhs, op, rhs } => {
                let mut result = Value::None();
                let Node::Identifier(id) = lhs.as_ref() else {
                    dbg!(node);
                    panic!("Expected Identifier node");
                };
                
                
                let Node::Expression(root) = rhs.as_ref() else {
                    dbg!(node);
                    panic!("Expected Expression node");
                };
                
                let Node::FunctionCall { id: func_id, arguments } = root.as_ref() else {
                    dbg!(node);
                    panic!("Expected FunctionCall node");
                };
                
                let Some(argus) = arguments else {
                    dbg!(node);
                    panic!("Expected arguments");
                };
                
                let mut argus = argus.clone();
                argus.insert(0, Node::Identifier(id.clone()));
                
                self.try_find_and_execute_fn(&Some(argus), func_id);
                
                result
            },
            Node::AddOp(lhs, rhs)
            | Node::SubOp(lhs, rhs)
            | Node::MulOp(lhs, rhs)
            | Node::DivOp(lhs, rhs) => {
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
            let function = Variable {
                mutable: *mutable,
                value: Value::Function(Rc::new(func)),
                m_type,
            };
            self.context.insert_variable(&id, Rc::new(function));
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
            typename,
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
                let Some(m_type) = self.type_checker.get(typename) else {
                    panic!("{} not a valid type", typename);
                };
                let var = Variable::new(*elements_mutable, val, m_type);
                values.push(var);
            }

            return Value::Array(*mutability, values);
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

        let var = match self.context.find_variable(id) {
            Some(var) => var,
            None => panic!("variable {:?} not found", id),
        };

        let (mutable, mut elements) = match var.value.clone() {
            Value::Array(mutable, elements) => (mutable, elements),
            _ => {
                dbg!(node);
                panic!("Expected Array node");
            }
        };

        let value_node = index.accept(self);

        if !mutable && *assignment {
            panic!("Cannot assign to immutable array");
        }

        let index_value = match value_node {
            Value::Double(index_value) => index_value as usize,
            Value::Int(index_value) => index_value as usize,
            _ => panic!("Expected numerical index value, got {:?}", value_node),
        };

        if elements.len() < index_value as usize {
            panic!(
                "Array index out of bounds :: {}[{}]",
                id, index_value as usize
            );
        }

        let element = &mut elements[index_value as usize];

        // read
        if !*assignment {
            return element.value.clone();
        }

        // assignment
        if let Some(expr) = expression {
            let expr_result = expr.accept(self);
            element.value = expr_result;
            if !TypeChecker::validate(&element) {
                dbg!(&element);
                panic!("invalid type");
            }
            let var2 = Variable::new(
                var.mutable,
                Value::Array(mutable, elements),
                Rc::clone(&var.m_type),
            );
            self.context.insert_variable(id, Rc::new(var2));
            return Value::None();
        }

        panic!("Expected expression in array assignment");
    }
}
