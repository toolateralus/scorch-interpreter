use std::collections::HashMap;

use crate::frontend::ast::{Node, Visitor};
use crate::frontend::tokens::TokenKind;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValueEnum;

pub enum Type {
    Int,
    Void,
    Bool,
    Float,
    String,
    Array { typename: String },
    Struct { id: String },
    Function { id: String },
}

pub struct LLVMLoweringVisitor<'ctx> {
    pub context: &'ctx Context,
    pub module: &'ctx Module<'ctx>,
    pub builder: &'ctx Builder<'ctx>,
    pub symbol_table: &'ctx mut SymbolTable<'ctx>,
}
pub struct SymbolTable<'ctx> {
    pub symbols: HashMap<String, BasicValueEnum<'ctx>>,
    pub functions: HashMap<String, FunctionDefinition<'ctx>>,
    pub structs: HashMap<String, StructDefinition<'ctx>>,
}
impl<'ctx> SymbolTable<'ctx> {
    // Add a new symbol
    pub fn add_symbol(&mut self, name: String, value: BasicValueEnum<'ctx>) {
        self.symbols.insert(name, value);
    }

    // Retrieve a symbol
    pub fn get_symbol(&self, name: &str) -> Option<&BasicValueEnum<'ctx>> {
        self.symbols.get(name)
    }

    // Add a new function
    pub fn add_function(&mut self, name: String, function: FunctionDefinition<'ctx>) {
        self.functions.insert(name, function);
    }

    // Retrieve a function
    pub fn get_function(&self, name: &str) -> Option<&FunctionDefinition<'ctx>> {
        self.functions.get(name)
    }

    // Add a new struct
    pub fn add_struct(&mut self, name: String, structure: StructDefinition<'ctx>) {
        self.structs.insert(name, structure);
    }

    // Retrieve a struct
    pub fn get_struct(&self, name: &str) -> Option<&StructDefinition<'ctx>> {
        self.structs.get(name)
    }
}
pub struct FunctionDefinition<'ctx> {
    pub name: String,
    pub params: HashMap<String, &'ctx Type>,
    pub return_type: &'ctx Type,
}
pub struct StructDefinition<'ctx> {
    pub name: String,
    pub fields: HashMap<String, &'ctx Type>,
}

impl<'ctx> Visitor<BasicValueEnum<'ctx>> for LLVMLoweringVisitor<'ctx> {
    fn visit_eof(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_declaration(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_program(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_block(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    // Operands
    fn visit_bool(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_string(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
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
            Node::Identifier(name) => match self.symbol_table.get_symbol(name) {
                Some(value) => *value,
                None => panic!("Undefined variable"),
            },
            _ => panic!("Expected Identifier node"),
        }
    }

    // Expressions
    fn visit_expression(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_term(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_factor(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    fn visit_relational_expression(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_logical_expression(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_not_op(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_neg_op(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_binary_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::BinaryOperation(lhs, op, rhs) => {
                let left = lhs.accept(self);
                let right = rhs.accept(self);

                match op {
                    TokenKind::Add => BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_add(
                                left.into_float_value(),
                                right.into_float_value(),
                                "addtmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::Subtract => BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_sub(
                                left.into_float_value(),
                                right.into_float_value(),
                                "subtmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::Multiply => BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_mul(
                                left.into_float_value(),
                                right.into_float_value(),
                                "multmp",
                            )
                            .unwrap(),
                    ),
                    TokenKind::Divide => BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_div(
                                left.into_float_value(),
                                right.into_float_value(),
                                "divtmp",
                            )
                            .unwrap(),
                    ),
                    _ => panic!("Unsupported binary operator"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }
    fn visit_assignment(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::Assignment { id, expression } => {
                let value = expression.accept(self);
                self.symbol_table.add_symbol(id.clone(), value);
                value
            }
            _ => panic!("Expected Assignment node"),
        }
    }

    // Functions
    fn visit_lambda(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_function_decl(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_param_decl(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_function_call(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    // Arrays.
    fn visit_array(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_array_access(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }

    // Keywords.
    fn visit_if_stmnt(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_else_stmnt(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_repeat_stmnt(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_break_stmnt(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
}
