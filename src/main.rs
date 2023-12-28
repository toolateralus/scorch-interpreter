// tokenizer, parser.
pub mod llvm;
pub mod util;

use ::std::collections::HashMap;
use ::std::env;
use scorch_parser::lexer::TokenProcessor;
use inkwell::context::Context;
use std::{fs, os};

use crate::llvm::context::SymbolTable;
use crate::llvm::lowering::LLVMVisitor;

fn main() {
    let flags_map = parse_cmd_line_args();
    let flags = util::Flags::new(flags_map);
    
    let file_path = format!("{}/{}", flags.proj_root, "scorch_src/main.scorch");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let mut tokenizer = scorch_parser::lexer::create_tokenizer();
    tokenizer.tokenize(&file_contents);
    let ast_root = scorch_parser::parser::parse_program(&tokenizer.tokens);
    
    let mut context = Context::create();
    let mut symbol_table = SymbolTable {
        symbols: HashMap::new(),
        functions: HashMap::new(),
        structs: HashMap::new(),
    };
    
    let mut visitor = LLVMVisitor::new(&mut context, &mut symbol_table);
    
    dbg!(&ast_root);
    let result = ast_root.accept(&mut visitor);
    
    println!("Result:");
    dbg!(&result);
    
    visitor.module.print_to_file("output.ll").unwrap();
    
}

fn parse_cmd_line_args() -> HashMap<String, bool> {
    let mut flags = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].clone();
        flags.insert(arg, true);
        i += 1;
    }
    return flags;
}
