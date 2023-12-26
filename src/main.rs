// tokenizer, parser.
pub mod frontend;
pub mod llvm;
pub mod util;

use ::std::collections::HashMap;
use ::std::env;
use std::fs;
use inkwell::context::Context;
use frontend::tokens::TokenProcessor;

use crate::llvm::lowering::LLVMLoweringVisitor;

fn main() {
    let flags_map = parse_cmd_line_args();
    let flags = util::Flags::new(flags_map);
    
    let file_path = format!("{}/{}", flags.proj_root, "scorch_src/main.scorch");
    let file_contents = fs::read_to_string(file_path).unwrap();
    
    let mut tokenizer = frontend::tokens::create_tokenizer();
    tokenizer.tokenize(&file_contents);
    let ast_root = frontend::parser::parse_program(&tokenizer.tokens);
    
    let mut symbol_table = llvm::context::SymbolTable {
        symbols: HashMap::new(),
        functions: HashMap::new(),
        structs: HashMap::new(),
    };
    
    let context = Context::create();
    let builder = context.create_builder();
    
    let mut visitor = &mut LLVMLoweringVisitor {
        context: &context,
        builder: &builder,
        symbol_table: &mut symbol_table,
    };
    
    dbg!(&ast_root);
    
    let result = ast_root.accept(visitor);
    
    println!("Result:");
    dbg!(&result);
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
