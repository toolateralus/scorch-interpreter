pub mod cli;
pub mod context;
pub mod interpreter;
pub mod standard_functions;
pub mod typechecker;
pub mod types;

use ::std::collections::HashMap;
use interpreter::*;
use scorch_parser::{lexer::{*, self}, parser};
use types::Value;

#[cfg(test)]
pub mod test;

pub fn run<'a>(code : &'a String) -> Result<&'a Value, String> {
    
    let mut lexer = lexer::create_tokenizer();
    
    lexer.tokenize(code);
    
    let tokens = &lexer.tokens;
    
    let mut interpreter = Interpreter::new();
    
    let ast_root = parser::parse_program(&tokens);
        
        let Ok(ast_root) = ast_root else {
            let Err(err) = ast_root else {
                panic!("Failed to parse input:");
            };
            
            dbg!(err);
            panic!();
        };
        
    ast_root.accept(&mut interpreter);
    
    Ok(&Value::None())   
}

pub fn run_with_modules<'a>(code_array : Vec<String>) -> Result<Value, String> {
    
    let mut interpreter = Interpreter::new();
    
    let mut result : Option<Value> = None;    
    
    for code in code_array {
        let mut lexer = lexer::create_tokenizer();
    
        lexer.tokenize(&code);
        
        let tokens = &lexer.tokens;
        
        let ast_root = parser::parse_program(&tokens);
            
            let Ok(ast_root) = ast_root else {
                let Err(err) = ast_root else {
                    panic!("Failed to parse input:");
                };
                
                dbg!(err);
                panic!();
            };
            
        result = Some(ast_root.accept(&mut interpreter));
    }
    
    match result.clone() {
        Some(value) => return Ok(value),
        None => return Err("No result".to_string())
    }
}