use std::collections::HashMap;
use crate::frontend::ast::{Node, Visitor};
use crate::frontend::tokens::TokenKind;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};
use super::context::{Instance, SymbolTable, Type, FunctionDefinition};
pub struct LLVMVisitor<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
    pub symbol_table: &'ctx mut SymbolTable<'ctx>,
}
impl<'ctx> LLVMVisitor<'ctx> {
    pub(crate) fn new(
        context: &'ctx Context,
        symbol_table: &'ctx mut SymbolTable<'ctx>,
    ) -> LLVMVisitor<'ctx> {
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

        let function = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();
        let mut result: Option<BasicValueEnum<'ctx>> = None;

        for statement in statements {
            let basic_block = self.context.append_basic_block(function, "block");
            self.builder.position_at_end(basic_block);
            result = Some(statement.accept(self));
        }

        result.unwrap()
    }
    fn visit_program(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        if let Node::Program(statements) = node {
            // Create a new function to contain the code for the program
            let function_type = self.context.void_type().fn_type(&[], false);
            let function = self.module.add_function("main", function_type, None);

            // Create a new basic block to start insertion into
            let basic_block = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(basic_block);

            // Visit each statement in the program
            let mut result: Option<BasicValueEnum<'ctx>> = None;
            for statement in statements {
                result = Some(statement.accept(self));
            }

            // Build a return instruction in the end of the function
            let Ok(_) = self.builder.build_return(None) else {
                panic!("Failed to build return instruction");
            };

            // Return the result of the last statement
            result.unwrap()
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
                BasicValueEnum::IntValue(
                    self.context.bool_type().const_int(bool_value as u64, false),
                )
            }
            _ => panic!("Expected BoolLiteral node"),
        }
    }
    fn visit_expression(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        match node {
            Node::Expression(root) => root.accept(self),
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
            Node::RelationalExpression { lhs, op, rhs } => {
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
            Node::LogicalExpression { lhs, op, rhs } => {
                let left = lhs.accept(self);
                let right = rhs.accept(self);

                match op {
                    TokenKind::LogicalAnd => BasicValueEnum::IntValue(
                        self.builder
                            .build_and(left.into_int_value(), right.into_int_value(), "andtmp")
                            .unwrap(),
                    ),
                    TokenKind::LogicalOr => BasicValueEnum::IntValue(
                        self.builder
                            .build_or(left.into_int_value(), right.into_int_value(), "ortmp")
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
            Node::DeclStmt {
                target_type,
                id,
                expression,
                mutable,
            } => {
                if self.symbol_table.get_var(id).is_some() {
                    panic!("Redefinition of a variable : {id}");
                }
                
                let value = expression.accept(self);
                let symbol = Instance {
                    name: id.clone(),
                    type_: Type::from_string(target_type.clone()),
                    value,
                    mutable: *mutable,
                };

                self.symbol_table.insert_var(id.clone(), symbol);

                value
            }
            _ => panic!("Expected Declaration node"),
        }
    }
    fn visit_lambda(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!()
    }
    fn visit_eof(&mut self, _node: &Node) -> BasicValueEnum<'ctx> {
        todo!() // probably do nothing always. this doesnt seem to always come in.
    }
    fn visit_identifier(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        let Node::Identifier(id) = &node else {
            panic!("Expected Identifier node");
        };
        let Some(var) = self.symbol_table.get_var(id) else {
            panic!("Variable not found");
        };
        var.value
    }
    fn visit_binary_op(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        let Node::BinaryOperation(lhs, op, rhs) = node else {
            panic!("Expected binary operator node");
        };
        let left = lhs.accept(self);
        let right = rhs.accept(self);
        match op {
            TokenKind::Add => match (left, right) {
                (BasicValueEnum::IntValue(left_val), BasicValueEnum::IntValue(right_val)) => {
                    BasicValueEnum::IntValue(
                        self.builder
                            .build_int_add(left_val, right_val, "addtmp")
                            .unwrap(),
                    )
                }
                (BasicValueEnum::FloatValue(left_val), BasicValueEnum::FloatValue(right_val)) => {
                    BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_add(left_val, right_val, "addtmp")
                            .unwrap(),
                    )
                }
                _ => panic!("Unsupported operand types for addition"),
            },
            TokenKind::Subtract => match (left, right) {
                (BasicValueEnum::IntValue(left_val), BasicValueEnum::IntValue(right_val)) => {
                    BasicValueEnum::IntValue(
                        self.builder
                            .build_int_sub(left_val, right_val, "subtmp")
                            .unwrap(),
                    )
                }
                (BasicValueEnum::FloatValue(left_val), BasicValueEnum::FloatValue(right_val)) => {
                    BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_sub(left_val, right_val, "subtmp")
                            .unwrap(),
                    )
                }
                _ => panic!("Unsupported operand types for subtraction"),
            },
            TokenKind::Multiply => match (left, right) {
                (BasicValueEnum::IntValue(left_val), BasicValueEnum::IntValue(right_val)) => {
                    BasicValueEnum::IntValue(
                        self.builder
                            .build_int_mul(left_val, right_val, "multmp")
                            .unwrap(),
                    )
                }
                (BasicValueEnum::FloatValue(left_val), BasicValueEnum::FloatValue(right_val)) => {
                    BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_mul(left_val, right_val, "multmp")
                            .unwrap(),
                    )
                }
                _ => panic!("Unsupported operand types for multiplication"),
            },
            TokenKind::Divide => match (left, right) {
                (BasicValueEnum::IntValue(left_val), BasicValueEnum::IntValue(right_val)) => {
                    BasicValueEnum::IntValue(
                        self.builder
                            .build_int_signed_div(left_val, right_val, "divtmp")
                            .unwrap(),
                    )
                }
                (BasicValueEnum::FloatValue(left_val), BasicValueEnum::FloatValue(right_val)) => {
                    BasicValueEnum::FloatValue(
                        self.builder
                            .build_float_div(left_val, right_val, "divtmp")
                            .unwrap(),
                    )
                }
                _ => panic!("Unsupported operand types for division"),
            },
            _ => panic!("Unsupported binary operator"),
        }
    }
    fn visit_assignment(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        let Node::Assignment { id, expression } = node else {
            panic!("Expected Assignment node");
        };
        let value = expression.accept(self);
        
        let Some(var) = self.symbol_table.get_var(id) else {
            dbg!(id);
            panic!("Variable not found :: cannot assign undefined variables.");
        };
        
        if !var.mutable {
            dbg!(node);
            panic!("Cannot assign to immutable variable");
        }
        
        self.symbol_table.insert_var(
            id.clone(),
            Instance {
                name: var.name.clone(),
                type_: var.type_.clone(),
                value,
                mutable: var.mutable,
            },
        );
        value
    }
    fn visit_function_decl(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        let Node::FnDeclStmnt {
            id,
            body,
            params,
            return_type,
            mutable: _,
        } = node
        else {
            panic!("Expected FunctionDecl node");
        };
        
        let mut params_new = HashMap::new();
        
        for param in params {
            let Node::ParamDecl { varname ,typename } = param else {
                panic!("Expected ParamDecl node");  
            };
            params_new.insert(varname.clone(), Type::from_string(typename.to_string()));
        }
        
        let function_type = self.context.i32_type().fn_type(&[], false);
        let function_value = self.module.add_function(&id, function_type, None);
        
        let entry_block = self.context.append_basic_block(function_value, "entry");
        self.builder.position_at_end(entry_block);
        
        let result = body.accept(self);
        
        self.builder.build_return(Some(&result));
        
        self.symbol_table.insert_fn(
            id.clone(),
            FunctionDefinition {
                name: id.clone(),
                params: HashMap::new(),
                return_type: Type::Int,
                func_val : function_value
            },
        );
        
        result
    }
    fn visit_function_call(&mut self, node: &Node) -> BasicValueEnum<'ctx> {
        let Node::FunctionCall { id, arguments } = node else {
            panic!("Expected FunctionCall node");
        };

        let mut args = Vec::new();

        let Some(arguments) = arguments.as_ref() else {
            panic!("Expected arguments");
        };

        for arg in arguments {
            args.push(arg.accept(self));
        }

        let Some(function) = self.symbol_table.get_fn(id) else {
            panic!("Function not found");
        };

        let args: Vec<BasicMetadataValueEnum> = args
            .iter()
            .map(|arg| match arg {
                BasicValueEnum::IntValue(int) => BasicMetadataValueEnum::IntValue(*int),
                BasicValueEnum::FloatValue(float) => BasicMetadataValueEnum::FloatValue(*float),
                BasicValueEnum::PointerValue(ptr) => BasicMetadataValueEnum::PointerValue(*ptr),
                BasicValueEnum::ArrayValue(array) => BasicMetadataValueEnum::ArrayValue(*array),
                BasicValueEnum::StructValue(structure) => {
                    BasicMetadataValueEnum::StructValue(*structure)
                },
                BasicValueEnum::VectorValue(vector) => BasicMetadataValueEnum::VectorValue(*vector),
            })
            .collect();

        let function_value = self.module.get_function(&function.name).unwrap();
        
        let Ok(call) = self
            .builder
            .build_call(function_value, &args[..], "calltmp")
        else {
            panic!("Failed to build function call");
        };

        call.try_as_basic_value().left().unwrap()
    }

}
