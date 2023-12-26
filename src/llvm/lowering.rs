use std::collections::HashMap;

use crate::frontend::ast::{Node, Visitor};
use crate::frontend::tokens::TokenKind;
use inkwell::builder::Builder;
use inkwell::context::{Context, self};

use inkwell::module::Module;
use inkwell::values::BasicValueEnum;

use super::context::SymbolTable;

pub struct LLVMVisitor<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub symbol_table: &'ctx mut SymbolTable<'ctx>,
}

impl<'ctx> LLVMVisitor<'ctx> {
    pub(crate) fn new(context: &'ctx Context, symbol_table: &'ctx mut SymbolTable<'ctx>) -> LLVMVisitor<'ctx> {
        LLVMVisitor {
            context,
            builder: context.create_builder(),
            module: context.create_module("program"),
            symbol_table,
        }
    }
}
impl<'ctx> Visitor<BasicValueEnum<'ctx>> for LLVMVisitor<'ctx> {
    
    fn visit_block(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        
        
        let Node::Block(statements) = node else {
            dbg!(node);
            panic!("Expected Block node");
        };
        let mut result : Option<BasicValueEnum<'ctx>> = None;
        for statement in statements {
            result = Some(statement.accept(self));
        }
        result.unwrap()
    }
    fn visit_program(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        
        if let Node::Program(statements) = node {
            let mut result: Option<BasicValueEnum<'ctx>> = None;
            for statement in statements {
                result = Some(statement.accept(self));
            }
            return result.unwrap();
        } else {
            panic!("Expected Program node");
        }
    }
    fn visit_string(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::String(string) => {
                let Ok(string_ptr) = self.builder.build_global_string_ptr(string, "string") else {
                    panic!("Failed to build string pointer");
                };
                BasicValueEnum::PointerValue(string_ptr.as_pointer_value())
            }
            _ => panic!("Expected StringLiteral node"),
        }
    }
    fn visit_bool(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::Bool(value) => {
                let bool_value = if *value { 1 } else { 0 };
                BasicValueEnum::IntValue(self.context.bool_type().const_int(bool_value as u64, false))
            }
            _ => panic!("Expected BoolLiteral node"),
        }
    }
    fn visit_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::Expression(root) => {
                root.accept(self)
            }
            _ => panic!("Expected Expression node"),
        }
    }
    fn visit_number(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::Int(value) => {
                let signed = true;
                BasicValueEnum::IntValue(self.context.i32_type().const_int(*value, signed))
            }
            Node::Double(value) => {
                BasicValueEnum::FloatValue(self.context.f64_type().const_float(*value))
            }
            _ => panic!("Expected Number node"),
        }
    }
    fn visit_relational_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::RelationalExpression{lhs, op, rhs} => {
                let left = lhs.accept(self);
                let right = rhs.accept(self);
                
                match op {
                    TokenKind::LeftAngle => BasicValueEnum::IntValue(
                        self.builder
                            .build_int_compare(
                                inkwell::IntPredicate::SLT,
                                left.into_int_value(),
                                right.into_int_value(),
                                "cmptmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::LessThanEquals => BasicValueEnum::IntValue(
                        self.builder
                            .build_int_compare(
                                inkwell::IntPredicate::SLE,
                                left.into_int_value(),
                                right.into_int_value(),
                                "cmptmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::RightAngle => BasicValueEnum::IntValue(
                        self.builder
                            .build_int_compare(
                                inkwell::IntPredicate::SGT,
                                left.into_int_value(),
                                right.into_int_value(),
                                "cmptmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::GreaterThanEquals => BasicValueEnum::IntValue(
                        self.builder
                            .build_int_compare(
                                inkwell::IntPredicate::SGE,
                                left.into_int_value(),
                                right.into_int_value(),
                                "cmptmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::Equals => BasicValueEnum::IntValue(
                        self.builder
                            .build_int_compare(
                                inkwell::IntPredicate::EQ,
                                left.into_int_value(),
                                right.into_int_value(),
                                "cmptmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::NotEquals => BasicValueEnum::IntValue(
                        self.builder
                            .build_int_compare(
                                inkwell::IntPredicate::NE,
                                left.into_int_value(),
                                right.into_int_value(),
                                "cmptmp",
                            )
                            .unwrap(),
                    ),
                    _ => panic!("Unsupported relational operator"),
                }
            }
            _ => panic!("Expected RelationalExpression node"),
        }
    }
    fn visit_logical_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::LogicalExpression{lhs, op, rhs} => {
                let left = lhs.accept(self);
                let right = rhs.accept(self);
                
                match op {
                    TokenKind::LogicalAnd => BasicValueEnum::IntValue(
                        self.builder
                            .build_and(
                                left.into_int_value(),
                                right.into_int_value(),
                                "andtmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::LogicalOr => BasicValueEnum::IntValue(
                        self.builder
                            .build_or(
                                left.into_int_value(),
                                right.into_int_value(),
                                "ortmp",
                            )
                            .unwrap(),
                    ),
                    _ => panic!("Unsupported logical operator"),
                }
            }
            _ => panic!("Expected LogicalExpression node"),
        }
    }
    fn visit_not_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::NotOp(expr) => {
                let value = expr.accept(self);
                BasicValueEnum::IntValue(
                    self.builder
                        .build_not(value.into_int_value(), "nottmp")
                        .unwrap(),
                )
            }
            _ => panic!("Expected NotOperation node"),
        }
    }
    fn visit_neg_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::NegOp(expr) => {
                let value = expr.accept(self);
                BasicValueEnum::FloatValue(
                    self.builder
                        .build_float_neg(value.into_float_value(), "negtmp")
                        .unwrap(),
                )
            }
            _ => panic!("Expected NegOperation node"),
        }
    }
    fn visit_declaration(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::DeclStmt { target_type: _, id, expression, mutable: _ } => {
                let value = expression.accept(self);
                self.symbol_table.add_symbol(id.clone(), value);
                value
            }
            _ => panic!("Expected Declaration node"),
        }
    }
    fn visit_lambda(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_eof(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    
    fn visit_identifier(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_binary_op(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_assignment(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
}
