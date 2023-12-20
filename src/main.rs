pub mod ast;
pub mod runtime;
pub mod tokens;
pub mod types;
pub mod bin_op;

#[cfg(test)]
pub mod test;

use runtime::Interpreter;
use std::fs::File;
use std::io::Read;
use std::{collections::HashMap, env};
use tokens::*;
use types::Context;

fn main() -> () {
    let flags = parse_cmd_line_args();

    if flags.len() == 0 {
        println!("Usage: scorch [options]");
        println!("Options:");
        println!("  --dump: dump tokens, ast, and global context");
        return;
    }

    let file = "test_functions.scorch";
    if flags.contains_key("dump") {
        execute_file_and_dump(String::from(file));
    } else {
        execute_return_global_ctx(String::from(file));
    }
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

fn execute_return_global_ctx(filename: String) -> Box<Context> {
    let mut tokenizer = tokens::create_tokenizer();
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());

    let tokens = tokenizer.tokens;
    let ast_root = ast::parse_program(&tokens);
    let mut interpreter = Interpreter {
        context: types::Context::new(),
    };

    ast_root.accept(&mut interpreter);

    let ctx = interpreter.context;
    return Box::new(ctx);
}
fn execute_file_and_dump(filename: String) {
    let mut tokenizer = tokens::create_tokenizer();
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());
    let tokens = tokenizer.tokens;
    println!("Tokens:");
    dbg!(&tokens);
    let ast_root = ast::parse_program(&tokens);
    println!("AST Root:");
    dbg!(&ast_root);
    let mut interpreter = Interpreter {
        context: types::Context::new(),
    };
    ast_root.accept(&mut interpreter);
    println!("Global Context:");
    dbg!(interpreter.context);
}
