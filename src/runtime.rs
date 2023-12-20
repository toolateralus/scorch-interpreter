use super::types::*;
use crate::{
    ast::{Node, Visitor},
    tokens::TokenKind,
};
#[derive(Debug)]
pub struct Interpreter {
    pub context: Context,
}
impl Visitor<ValueType> for Interpreter {
    fn visit_block(&mut self, node: &Node) -> ValueType {
        if let Node::Block(statements) = node {
            for statement in statements {
                statement.accept(self);
            }
        } else {
            panic!("Expected Block node");
        }
        return ValueType::None(());
    }
    fn visit_declaration(&mut self, node: &Node) -> ValueType {
        if let Node::DeclStmt {
            target_type,
            id,
            expression,
        } = node
        {
            let value: ValueType;

            match target_type.as_str() {
                "dynamic" | "num" | "string" => {
                    // todo: add an actual type system.
                    value = expression.accept(self);
                }
                _ => {
                    dbg!(node);
                    panic!("Unsupported type");
                }
            }

            let str_id: String = id.clone();

            // redefinition
            if self.context.variables.contains_key(&str_id) {
                dbg!(node);
                panic!("redefinition of variable");
            } else {
                self.context.variables.insert(str_id, Box::new(value));
            }
        } else {
            panic!("Expected Declaration node");
        }
        ValueType::None(())
    }
    fn visit_identifier(&mut self, node: &Node) -> ValueType {
        let Node::Identifier(id) = node else {
            dbg!(node);
            panic!("Expected Identifier");
        };
        match self.context.variables.get(id) {
            Some(value) => *value.clone(), // todo : fix copy on value type references here.
            None => {
                dbg!(node);
                panic!("Variable not found");
            }
        }
    }
    fn visit_number(&mut self, node: &Node) -> ValueType {
        let Node::Number(value) = node else {
            dbg!(node);
            panic!("Expected Number");
        };
        ValueType::Float(*value)
    }
    fn visit_term(&mut self, _node: &Node) -> ValueType {
        return ValueType::None(());
    }
    fn visit_factor(&mut self, node: &Node) -> ValueType {
        match node {
            Node::Expression(root) => root.accept(self),
            _ => {
                dbg!(node);
                panic!("Expected Number or Identifier node");
            }
        }
    }
    fn visit_assignment(&mut self, node: &Node) -> ValueType {
        match node {
            Node::AssignStmnt { id, expression } => {
                let val: ValueType;
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
                        *value = Box::new(val.clone());
                    }
                    None => {
                        dbg!(node);
                        panic!("Variable not found");
                    }
                }
                return ValueType::None(());
            }
            _ => {
                dbg!(node);
                panic!("Expected Assignment node");
            }
        }
    }
    fn visit_binary_op(&mut self, node: &Node) -> ValueType {
        match node {
            Node::AddOp(lhs, rhs)
            | Node::SubOp(lhs, rhs)
            | Node::MulOp(lhs, rhs)
            | Node::DivOp(lhs, rhs) => {
                let e_lhs = lhs.accept(self);
                let e_rhs = rhs.accept(self);
                match (e_lhs, e_rhs) {
                    (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => {
                        return self.bin_op_float(node, &lhs_float, &rhs_float);
                    }
                    (ValueType::String(lhs_string), ValueType::String(rhs_string)) => {
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
    fn visit_string(&mut self, node: &Node) -> ValueType {
        if let Node::String(value) = node {
            return ValueType::String(value.clone());
        } else {
            panic!("Expected String node");
        }
    }
    fn visit_expression(&mut self, node: &Node) -> ValueType {
        if let Node::Expression(root) = node {
            return root.accept(self);
        } else {
            panic!("Expected Expression node");
        }
    }
    fn visit_eof(&mut self, _node: &Node) -> ValueType {
        ValueType::None(()) // do nothing.
    }
    fn visit_not_op(&mut self, node: &Node) -> ValueType {
        if let Node::NotOp(operand) = node {
            match operand.accept(self) {
                ValueType::Bool(value) => ValueType::Bool(!value),
                ValueType::Float(mut value) => {
                    value = 1.0 - value;
                    if value > 1.0 {
                        value = 1.0;
                    } else if value < 0.0 {
                        value = 0.0;
                    }
                    ValueType::Float(value)
                }
                _ => panic!("Expected boolean or numerical operand for not operation"),
            }
        } else {
            panic!("Expected NotOp node");
        }
    }
    fn visit_neg_op(&mut self, node: &Node) -> ValueType {
        if let Node::NegOp(operand) = node {
            match operand.accept(self) {
                ValueType::Float(value) => ValueType::Float(-value),
                _ => panic!("Expected numeric operand for negation operation"),
            }
        } else {
            panic!("Expected NegOp node");
        }
    }
    fn visit_bool(&mut self, node: &Node) -> ValueType {
        if let Node::Bool(value) = node {
            return ValueType::Bool(*value);
        } else {
            panic!("Expected Bool node");
        }
    }
    fn visit_if_stmnt(&mut self, node: &Node) -> ValueType {
        if let Node::IfStmnt {
            condition,
            block: true_block,
            else_stmnt: else_block,
        } = node
        {
            if let ValueType::Bool(condition_result) = condition.accept(self) {
                if condition_result {
                    true_block.accept(self);
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
        return ValueType::None(());
    }
    fn visit_else_stmnt(&mut self, node: &Node) -> ValueType {
        match node {
            Node::ElseStmnt {
                condition,
                block: true_block,
                else_stmnt,
            } => {
                let condition_result = match condition.as_ref() {
                    Some(expression) => {
                        if let ValueType::Bool(val) = expression.accept(self) {
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

        ValueType::None(())
    }
    fn visit_relational_expression(&mut self, node: &Node) -> ValueType {
        if let Node::RelationalExpression { lhs, op, rhs } = node {
            let lhs_value = lhs.accept(self);
            let rhs_value = rhs.accept(self);
            match (lhs_value, rhs_value) {
                (ValueType::Bool(lhs_bool), ValueType::Bool(rhs_bool)) => match op {
                    TokenKind::Equals => return ValueType::Bool(lhs_bool == rhs_bool),
                    TokenKind::NotEquals => return ValueType::Bool(lhs_bool != rhs_bool),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                (ValueType::Float(lhs_float), ValueType::Float(rhs_float)) => match op {
                    TokenKind::LeftAngle => return ValueType::Bool(lhs_float < rhs_float),
                    TokenKind::LessThanEquals => return ValueType::Bool(lhs_float <= rhs_float),
                    TokenKind::RightAngle => return ValueType::Bool(lhs_float > rhs_float),
                    TokenKind::GreaterThanEquals => return ValueType::Bool(lhs_float >= rhs_float),
                    TokenKind::Equals => return ValueType::Bool(lhs_float == rhs_float),
                    TokenKind::NotEquals => return ValueType::Bool(lhs_float != rhs_float),
                    _ => {
                        dbg!(node);
                        panic!("invalid operator");
                    }
                },
                (ValueType::String(lhs_string), ValueType::String(rhs_string)) => match op {
                    TokenKind::Equals => return ValueType::Bool(lhs_string == rhs_string),
                    TokenKind::NotEquals => return ValueType::Bool(lhs_string != rhs_string),
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
    fn visit_logical_expression(&mut self, node: &Node) -> ValueType {
        if let Node::LogicalExpression { lhs, op, rhs } = node {
            let lhs_value = lhs.accept(self);
            let rhs_value = rhs.accept(self);
            match (lhs_value, rhs_value) {
                (ValueType::Bool(lhs_bool), ValueType::Bool(rhs_bool)) => match op {
                    TokenKind::LogicalAnd => return ValueType::Bool(lhs_bool && rhs_bool),
                    TokenKind::LogicalOr => return ValueType::Bool(lhs_bool || rhs_bool),
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
    fn visit_function_decl(&mut self, node: &Node) -> ValueType {
        if let Node::FnDeclStmnt {
            id,
            params,
            body,
            return_type,
        } = node
        {
            let body_cloned = body.clone();
            let func = Function {
                name: id.clone(),
                params: self.get_params_list(params),
                body: body_cloned,
                return_type: return_type.clone(),
            };
            let function = Box::new(func);
            self.context.functions.insert(id.clone(), function);
        } else {
            panic!("Expected FunctionDecl node");
        };
        ValueType::None(())
    }

    fn visit_param_decl(&mut self, _node: &Node) -> ValueType {
        todo!()
    }
    fn visit_program(&mut self, node: &Node) -> ValueType {
        if let Node::Program(statements) = node {
            for stmnt in statements {
                stmnt.accept(self);
            }
        } else {
            panic!("expected program node");
        };
        ValueType::None(())
        // this is unused since it uses a different return type. see impl Interpeter.
    }

    fn visit_function_call(&mut self, node: &Node) -> ValueType {
        let return_value = ValueType::None(());
        if let Node::FunctionCall { id, arguments } = node {
            if self.context.variables.contains_key(id) {
                let old = self.context.clone();
                let function = old.functions.get(id).unwrap();
                let args = Function::create_args(self, arguments, &old);
                let ctx = Context::new();

                if args.len() == 0 {
                    return function.body.accept(self);
                }

                // Check if the number of arguments matches the number of parameters
                if args.len() != function.params.len() {
                    panic!("Number of arguments does not match the number of parameters");
                }

                // Check if the types and order of arguments match the parameters
                for (arg, param) in args.iter().zip(function.params.iter()) {
                    let arg_type_name = match *arg {
                        ValueType::Float(_) => "num",
                        ValueType::Bool(_) => "bool",
                        ValueType::String(_) => "string",
                        ValueType::None(_) => "undefined",
                    };
                    if arg_type_name != param.name {
                        panic!("Argument type does not match parameter type")
                    } else {
                        self.context
                            .variables
                            .insert(param.name.clone(), Box::new(arg.clone()));
                    }
                }

                self.context = ctx;
                function.body.accept(self);
            }
        }
        return_value
    }
}

// binary operation definitions
impl Interpreter {
    fn get_params_list(&mut self, param_nodes: &Vec<Node>) -> Vec<Parameter> {
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

                let parameter = Parameter {
                    name: param_name,
                    typename: type_name,
                };

                params.push(parameter);
            }
        }
        params
    }
    fn bin_op_float(&mut self, node: &Node, lhs: &f64, rhs: &f64) -> ValueType {
        let result: f64;
        match node {
            Node::AddOp(_, _) => result = lhs + rhs,
            Node::SubOp(_, _) => result = lhs - rhs,
            Node::MulOp(_, _) => result = lhs * rhs,
            Node::DivOp(_, _) => result = lhs / rhs,
            _ => {
                dbg!(node);
                panic!("Expected binary operation node");
            }
        }
        ValueType::Float(result)
    }
    fn bin_op_string(&mut self, node: &Node, lhs: &String, rhs: &String) -> ValueType {
        let result: String;
        match node {
            Node::AddOp(_, _) => result = format!("{}{}", lhs, rhs),
            _ => {
                dbg!(node);
                panic!("invalid binary operation on strings");
            }
        }
        ValueType::String(result)
    }
}
