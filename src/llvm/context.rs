use std::collections::HashMap;
use inkwell::values::BasicValueEnum;
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

pub struct SymbolTable<'ctx> {
    pub symbols: HashMap<String, Instance<'ctx>>,
    pub functions: HashMap<String, FunctionDefinition>,
    pub structs: HashMap<String, StructDefinition>,
}

impl<'ctx> SymbolTable<'ctx> {
    pub fn insert_var(&mut self, name: String, value: Instance<'ctx>) {
        self.symbols.insert(name, value);
    }
    pub fn get_var(&self, name: &str) -> Option<&Instance<'ctx>> {
        self.symbols.get(name)
    }
    pub fn insert_fn(&mut self, name: String, function: FunctionDefinition) {
        self.functions.insert(name, function);
    }
    pub fn get_fn(&self, name: &str) -> Option<&FunctionDefinition> {
        self.functions.get(name)
    }
    pub fn insert_struct(&mut self, name: String, structure: StructDefinition) {
        self.structs.insert(name, structure);
    }
    pub fn get_struct(&self, name: &str) -> Option<&StructDefinition> {
        self.structs.get(name)
    }
}

pub struct Instance<'ctx> {
    pub name: String,
    pub type_: Type,
    pub value: BasicValueEnum<'ctx>,
}

pub struct FunctionDefinition {
    pub name: String,
    pub params: HashMap<String, Type>,
    pub return_type: Type,
}
pub struct StructDefinition {
    pub name: String,
    pub fields: HashMap<String,Type>,
}
