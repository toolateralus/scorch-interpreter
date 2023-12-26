use crate::frontend::ast::{Visitor, Node};
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;

pub struct LLVMLoweringVisitor<'ctx> {
    context: &'ctx Context,
    // Add any other fields you need here
}

impl<'ctx> Visitor<BasicValueEnum<'ctx>> for LLVMLoweringVisitor<'ctx> {
    fn visit_number(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    
    fn visit_term(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_factor(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_eof(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_binary_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_lambda(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_function_decl(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_program(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_relational_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_logical_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_not_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_neg_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_assignment(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_declaration(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_block(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_string(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_identifier(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_bool(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_array(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_array_access(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_if_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_else_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_param_decl(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_function_call(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_repeat_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_break_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
}