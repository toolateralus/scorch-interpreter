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
    pub functions: HashMap<String, FunctionDefinition<'ctx>>,
    pub structs: HashMap<String, StructDefinition<'ctx>>,
}
impl<'ctx> SymbolTable<'ctx> {
    pub fn add_symbol(&mut self, name: String, value: BasicValueEnum<'ctx>) {
        self.symbols.insert(name, value);
    }
    pub fn get_symbol(&self, name: &str) -> Option<&BasicValueEnum<'ctx>> {
        self.symbols.get(name)
    }
    pub fn add_function(&mut self, name: String, function: FunctionDefinition<'ctx>) {
        self.functions.insert(name, function);
    }
    pub fn get_function(&self, name: &str) -> Option<&FunctionDefinition<'ctx>> {
        self.functions.get(name)
    }
    pub fn add_struct(&mut self, name: String, structure: StructDefinition<'ctx>) {
        self.structs.insert(name, structure);
    }
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
