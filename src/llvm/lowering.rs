use crate::frontend::ast::{Visitor, Node};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

pub enum Type {
    Int,
    Void,
    Bool,
    Float,
    String,
    Array{typename: String},
    Struct{id: String},
    Function{id: String},
}

pub struct LLVMLoweringVisitor<'ctx> {
    pub context: &'ctx Context,
    pub module : &'ctx Module<'ctx>,
    pub builder: &'ctx Builder<'ctx>,
}

pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<String>,
}

impl<'ctx> Visitor<BasicValueEnum<'ctx>> for LLVMLoweringVisitor<'ctx> {
    fn visit_eof(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    
    fn visit_declaration(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_program(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_block(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    
    // Operands
    fn visit_bool(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_string(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_number(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match &node {
            Node::Double(dbl) => {
                BasicValueEnum::FloatValue(self.context.f64_type().const_float(*dbl))
            }
            Node::Int(int) => {
                BasicValueEnum::IntValue(self.context.i32_type().const_int(*int, true))
            }
            _ => {
                panic!("Expected number")
            }
        }
    }
    fn visit_identifier(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::Identifier(name) => {
                match self.symbol_table.get(name) {
                    Some(value) => *value,
                    None => panic!("Undefined variable"),
                }
            }
            _ => panic!("Expected Identifier node"),
        }
    }
        
    // Expressions
    fn visit_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_term(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_factor(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_binary_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
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
    fn visit_binary_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::BinaryOp { left, op, right } => {
                let left = self.visit(left);
                let right = self.visit(right);
                
                match op.as_str() {
                    "+" => self.builder.build_float_add(left.into_float_value(), right.into_float_value(), "addtmp"),
                    "-" => self.builder.build_float_sub(left.into_float_value(), right.into_float_value(), "subtmp"),
                    "*" => self.builder.build_float_mul(left.into_float_value(), right.into_float_value(), "multmp"),
                    "/" => self.builder.build_float_div(left.into_float_value(), right.into_float_value(), "divtmp"),
                    _ => panic!("Unsupported binary operator"),
                }.into()
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }
    fn visit_assignment(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::Assignment { name, value } => {
                let value = self.visit(value);

                self.symbol_table.insert(name.clone(), value);

                value
            }
            _ => panic!("Expected Assignment node"),
        }
    }
    
    // Functions
    fn visit_lambda(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_function_decl(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_param_decl(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_function_call(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    
    // Arrays.
    fn visit_array(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_array_access(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    
    // Keywords.
    fn visit_if_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_else_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_repeat_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_break_stmnt(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
}