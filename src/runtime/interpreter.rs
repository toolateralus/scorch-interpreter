use std::{collections::HashMap, rc::Rc};

use super::{types::*, typechecker::TypeChecker};
use crate::frontend::{
    ast::{Node, Visitor},
    tokens::TokenKind,
};

pub struct Interpreter {
    pub context: Context, // initally the root context, but this is a kinda tree like structure.
    pub builtin: HashMap<String, BuiltInFunction>,
    pub type_checker : TypeChecker,
}
impl Interpreter {
    pub fn new() -> Interpreter {
        let builtins = super::expression::get_builtin_functions();
        Interpreter {
            context: Context::new(),
            builtin: builtins,
            type_checker : TypeChecker::new(),
        }
    }
}

impl Visitor<Value> for Interpreter {
    // top level nodes
    fn visit_program(&mut self, node: &Node) -> Value {
        if let Node::Program(statements) = node {
            for stmnt in statements {
                let result = stmnt.accept(self);
                if let Value::Return(ret_val) = result {
                    if let Some(return_value) = ret_val {
                        return *return_value;
                    } else {
                        return Value::None(());
                    }
                }
            }
        } else {
            panic!("expected program node");
        };
        Value::None(())
        // this is unused since it uses a different return type. see impl Interpeter.
    }
    fn visit_block(&mut self, node: &Node) -> Value {
        if let Node::Block(statements) = node {
            for statement in statements {
                let value = statement.accept(self);
                if let Value::Return(ret_val) = value {
                    if let Some(return_value) = ret_val {
                        return *return_value;
                    } else {
                        return Value::Return(None);
                    }
                }
            }
        } else {
            panic!("Expected Block node");
        }
        return Value::None(());
    }

    // statements
    fn visit_if_stmnt(&mut self, node: &Node) -> Value {
        if let Node::IfStmnt {
            condition,
            block: true_block,
            else_stmnt: else_block,
        } = node
        {
            if let Value::Bool(condition_result) = condition.accept(self) {
                if condition_result {
                    if let Node::Block(stmnts) = &**true_block {
                        for stmnt in stmnts {
                            let value = stmnt.accept(self);
                            if let Value::Return(_) = value {
                                return value;
                            }
                        }
                    }
                } else {
                    if let Some(else_stmnt) = else_block {
                        else_stmnt.accept(self);
                    }
                }
            } else {
                panic!("Expected boolean condition");
            }
        } else {
            panic!("Expected WhereStmnt node");
        }
        return Value::None(());
    }
    fn visit_else_stmnt(&mut self, node: &Node) -> Value {
        match node {
            Node::ElseStmnt {
                condition,
                block: true_block,
                else_stmnt,
            } => {
                let condition_result = match condition.as_ref() {
                    Some(expression) => {
                        if let Value::Bool(val) = expression.accept(self) {
                            val
                        } else {
                            panic!("Expected boolean condition");
                        }
                    }
                    None => true,
                };

                if condition_result {
                    true_block.accept(self);
                } else if let Some(else_statement) = else_stmnt {
                    else_statement.accept(self);
                } else {
                }
            }
            _ => panic!("Expected OrStmnt node"),
        }

        Value::None(())
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

            match target_type.as_str() {
                "Dynamic" | "Float" | "String" | "Bool" | "Struct" | "Array" => {
                    // todo: add an actual type system.
                    value = expression.accept(self);
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
                    let mutability = *mutable;
                    self.context.insert_variable(
                        &id,
                        Rc::new(Variable::from(target_type.clone(), mutability, value, self.type_checker.clone())),
                    );
                }
            }
        } else {
            panic!("Expected Declaration node");
        }
        Value::None(())
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
                        
                        if TypeChecker::validate(value, None) == false {
                            dbg!(node);
                            panic!("Type mismatch");
                        }
                        
                        
                        if value.mutable == false {
                            dbg!(node);
                            panic!("Cannot assign to immutable variable");
                        }
                        
                        *value = Rc::new(Variable::from(
                            value.typename.clone(),
                            value.mutable,
                            val,
                            self.type_checker.clone(),
                        ));
                    }
                    None => {
                        dbg!(node);
                        panic!("Variable not found");
                    }
                }
                return Value::None(());
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
        match self.context.find_function(id) {
            // create function pointer basically
            Some(func) => return Value::Function(func),
            None => {
                // to be consumed elsewhere.
            }
        }
        match self.context.find_variable(id) {
            Some(value) => (*value).value.clone(), // todo: fix cloning all values.
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
        let Node::Number(value) = node else {
            dbg!(node);
            panic!("Expected Number");
        };
        Value::Float(*value)
    }
    fn visit_string(&mut self, node: &Node) -> Value {
        if let Node::String(value) = node {
            return Value::String(value.clone());
        } else {
            panic!("Expected String node");
        }
    }
    fn visit_eof(&mut self, _node: &Node) -> Value {
        Value::None(()) // do nothing.
    }

    // unary operations
    fn visit_not_op(&mut self, node: &Node) -> Value {
        if let Node::NotOp(operand) = node {
            match operand.accept(self) {
                Value::Bool(value) => Value::Bool(!value),
                Value::Float(mut value) => {
                    value = 1.0 - value;
                    if value > 1.0 {
                        value = 1.0;
                    } else if value < 0.0 {
                        value = 0.0;
                    }
                    Value::Float(value)
                }
                _ => panic!("Expected boolean or numerical operand for not operation"),
            }
        } else {
            panic!("Expected NotOp node");
        }
    }
    fn visit_neg_op(&mut self, node: &Node) -> Value {
        if let Node::NegOp(operand) = node {
            match operand.accept(self) {
                Value::Float(value) => Value::Float(-value),
                _ => panic!("Expected numeric operand for negation operation"),
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
                (Value::Float(lhs_float), Value::Float(rhs_float)) => match op {
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
            Node::AddOp(lhs, rhs)
            | Node::SubOp(lhs, rhs)
            | Node::MulOp(lhs, rhs)
            | Node::DivOp(lhs, rhs) => {
                let e_lhs = lhs.accept(self);
                let e_rhs = rhs.accept(self);
                match (e_lhs, e_rhs) {
                    (Value::Float(lhs_float), Value::Float(rhs_float)) => {
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
        return Value::None(());
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
        let old = self.context.clone();
        if let Node::FunctionCall { id, arguments } = node {
            let args;
            let function;
            {
                args = Function::extract_args(self, arguments, &old);

                // builtin, written in Rust
                if self.builtin.contains_key(id) {
                    let builtin = self.builtin.get_mut(id).unwrap();
                    return builtin.call(args.clone());
                }
                // native function, written in Scorch.
                else if let Some(fn_ptr) = self.context.find_function(id) {
                    function = fn_ptr
                }
                // function pointer
                else if let Some(fn_ptr) = self.context.find_variable(id) {
                    if let Value::Function(func) = fn_ptr.value.clone() {
                        function = func.clone()
                    } else {
                        panic!("Expected function");
                    }
                } else {
                    dbg!(node);
                    panic!("Function not found");
                }
            }
            
            // parameterless invocation.
            if function.params.len() + args.len() == 0 {
                return function.body.accept(self);
            }

            // todo; varargs
            if args.len() != function.params.len() {
                panic!("Number of arguments does not match the number of parameters");
            }

            for (arg, param) in args.iter().zip(function.params.iter()) {
                let arg_type_name = get_type_name(arg);

                // typecheck args. very basic.
                if arg_type_name.to_string() != param.typename {
                    panic!("Argument type does not match parameter type.\n provided argument: {:?} expected parameter : {:?}", arg, param)
                } else {
                    // copying param values into a context
                    self.context.insert_variable(
                        &param.name,
                        Rc::new(Variable::from(
                            param.typename.clone(),
                            false,
                            arg.clone(),
                            self.type_checker.clone(),
                        )),
                    );
                }
            }

            let ret = function.body.accept(self);

            if let Value::Return(ret_val) = ret {
                if let Some(return_value) = ret_val {
                    return *return_value;
                }
            };
            // todo: don't discard changes made by functions
            // right now - side effects are undone on context leave.
            self.context = old;
        }
        Value::None(())
    }
    fn visit_function_decl(&mut self, node: &Node) -> Value {
        if let Node::FnDeclStmnt {
            id,
            params,
            body,
            return_type,
            mutable
        } = node
        {
            let body_cloned = body.clone();
            let func = Function {
                name: id.clone(),
                params: self.get_params_list(params),
                body: body_cloned,
                return_type: return_type.clone(),
                mutable: *mutable,
            };
            let function = Rc::new(func);
            self.context.insert_function(&id, function);
        } else {
            panic!("Expected FunctionDecl node");
        };
        Value::None(())
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
}

fn get_type_name<'a>(arg: &'a Value) -> &'a str {
    let arg_type_name = match arg {
        Value::Float(_) => "Float",
        Value::Bool(_) => "Bool",
        Value::String(_) => "String",
        Value::None(_) => "undefined",
        Value::Function(_func) => "function",
        _ => {
            dbg!(arg);
            panic!("invalid argument type")
        }
    };
    arg_type_name
}
