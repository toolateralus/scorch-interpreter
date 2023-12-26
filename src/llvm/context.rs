use inkwell::values::{BasicValueEnum, FunctionValue};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Void,
    Bool,
    Float,
    String,
    Function,
    Array { type_name: String },
    Struct { id: String },
    Dynamic,
}

impl Type {
    pub(crate) fn from_string(return_type: String) -> Type {
        match return_type.as_str() {
            "Int" => Type::Int,
            "Void" => Type::Void,
            "Bool" => Type::Bool,
            "Float" => Type::Float,
            "String" => Type::String,
            "Fn" => Type::Function,
            "Array" => Type::Array {   type_name: String::from("Dynamic") },
            "Struct" => Type::Struct { id: String::from("Dynamic") },
            _ => Type::Dynamic,
        }
    }
}

pub struct SymbolTable<'ctx> {
    pub symbols: HashMap<String, Instance<'ctx>>,
    pub functions: HashMap<String, FunctionDefinition<'ctx>>,
    pub structs: HashMap<String, StructDefinition>,
}

impl<'ctx> SymbolTable<'ctx> {
    pub fn insert_var(&mut self, name: String, value: Instance<'ctx>) {
        self.symbols.insert(name, value);
    }
    pub fn get_var(&self, name: &str) -> Option<&Instance<'ctx>> {
        self.symbols.get(name)
    }
    pub fn insert_fn(&mut self, name: String, function: FunctionDefinition<'ctx>) {
        self.functions.insert(name, function);
    }
    pub fn get_fn(&self, name: &str) -> Option<&FunctionDefinition> {
        self.functions.get(name)
    }
    // pub fn insert_struct(&mut self, name: String, structure: StructDefinition) {
    //     self.structs.insert(name, structure);
    // }
    // pub fn get_struct(&self, name: &str) -> Option<&StructDefinition> {
    //     self.structs.get(name)
    // }
}


pub struct Instance<'ctx> {
    pub name: String,
    pub type_: Type,
    pub value: BasicValueEnum<'ctx>,
    pub mutable: bool,
}

pub struct FunctionDefinition<'ctx> {
    pub name: String,
    pub params: HashMap<String, Type>,
    pub return_type: Type,
    pub func_val: FunctionValue<'ctx>,
}
pub struct StructDefinition {
    pub name: String,
    pub fields: HashMap<String, Type>,
}
