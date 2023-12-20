use std::fs::File;
use std::io::{self, Write, Read};

use crate::frontend::*;
use crate::*;


pub struct Flags {
    pub proj_root: String,
    pub dump: bool,
    pub cli: bool,
}
impl Flags {
    pub fn new(flags_map: HashMap<String, bool>) -> Flags {
        Flags {
            proj_root: get_project_root(),
            dump: flags_map.contains_key("dump"),
            cli: flags_map.contains_key("cli"),
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

pub fn run_cli() {
    let mut tokenizer = tokens::create_tokenizer();
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

        let ast_root = ast::parse_program(&tokens);
        ast_root.accept(&mut interpreter);
        input.clear();
    }
}
pub fn execute_from_file(filename: String) -> Box<Context> {
    let mut tokenizer = tokens::create_tokenizer();
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    tokenizer.tokenize(&contents.as_str());

    let tokens = tokenizer.tokens;
    let ast_root = ast::parse_program(&tokens);
    let mut interpreter = Interpreter::new();

    ast_root.accept(&mut interpreter);

    let ctx = interpreter.context;
    return Box::new(ctx);
}
pub fn execute_file_then_dump(filename: String) {
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
    let mut interpreter = Interpreter::new();
    ast_root.accept(&mut interpreter);
    println!("Global Context:");
    dbg!(interpreter.context);
}
