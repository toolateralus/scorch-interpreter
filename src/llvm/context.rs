use std::collections::HashMap;
use inkwell::values::BasicValueEnum;
pub enum Type {
    Int,
    Void,
    Bool,
    Float,
    String,
    Array { type_name: String },
    Struct { id: String },
    Function { id: String },
}

pub struct SymbolTable<'ctx> {
    pub symbols: HashMap<String, BasicValueEnum<'ctx>>,
    pub functions: HashMap<String, FunctionDefinition>,
    pub structs: HashMap<String, StructDefinition>,
}

impl<'ctx> SymbolTable<'ctx> {
    pub fn insert_var(&mut self, name: String, value: BasicValueEnum<'ctx>) {
        self.symbols.insert(name, value);
    }
    pub fn get_var(&self, name: &str) -> Option<&BasicValueEnum<'ctx>> {
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

pub struct FunctionDefinition {
    pub name: String,
    pub params: HashMap<String, Type>,
    pub return_type: Type,
}
pub struct StructDefinition {
    pub name: String,
    pub fields: HashMap<String,Type>,
}
