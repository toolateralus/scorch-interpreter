use std::cell::RefCell;
use std::fs::File;
use std::io::{self, Read, Write};
use std::rc::Rc;

use crate::context::Context;
use crate::*;
use scorch_parser::*;

pub struct Flags {
    pub proj_root: String,
    pub dump: bool,
    pub cli: bool,
    pub no_interpret: bool,
}
impl Flags {
    pub fn new(flags_map: HashMap<String, bool>) -> Flags {
        Flags {
            proj_root: get_project_root(),
            dump: flags_map.contains_key("dump"),
            cli: flags_map.contains_key("cli"),
            no_interpret: flags_map.contains_key("no-interpret"),
        }
    }
    pub fn qualify_from_root(&self, path: String) {
        let mut path = path;
        if path.starts_with("/") {
            path.remove(0);
        }
        path.insert_str(0, &self.proj_root);
    }
}
pub fn get_project_root() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let src_dir = current_dir.join("src");
    let parent_dir = src_dir.parent().expect("Failed to get project root");
    let project_root = parent_dir.to_path_buf();
    project_root.to_str().unwrap().to_string()
}

pub fn run_repl() {
    let mut tokenizer = lexer::create_tokenizer();
    let mut interpreter = Interpreter::new();

    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        tokenizer.tokenize(&input.as_str());
        let tokens = &tokenizer.tokens;
        
        let ast_root = parser::parse_program(&tokens);
        
        let Ok(ast_root) = ast_root else {
            let Err(err) = ast_root else {
                println!("Failed to parse input:");
                continue;
            };
            
            dbg!(err);
            panic!();
        };
        
        
        ast_root.accept(&mut interpreter);
        
        input.clear();
    }
}
pub fn execute_from_file(filename: String) -> Rc<RefCell<Context>> {
    let mut tokenizer = lexer::create_tokenizer();
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());

    let tokens = tokenizer.tokens;
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
    interpreter.context
}
pub fn execute_file_then_dump(filename: String) {
    let mut tokenizer = lexer::create_tokenizer();
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());
    let tokens = tokenizer.tokens;
    println!("Tokens:");
    dbg!(&tokens);
    let ast_root = parser::parse_program(&tokens);
    println!("AST Root:");
    dbg!(&ast_root);
    let mut interpreter = Interpreter::new();
    let Ok(ast_root) = ast_root else {
        panic!("Failed to parse input");
    };
    
    ast_root.accept(&mut interpreter);
    println!("Global Context:");
    
    dbg!(interpreter.type_checker.types);
    dbg!(interpreter.context);
}
